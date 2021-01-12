/*!
```rudra-poc
[target]
crate = "crayon"
version = "0.7.1"

[test]
analyzers = ["Manual", "UnsafeDestructor"]

[report]
issue_url = "https://github.com/shawnscode/crayon/issues/87"
issue_date = 2020-08-31
rustsec_url = "https://github.com/RustSec/advisory-db/pull/371"
rustsec_id = "RUSTSEC-2020-0037"
```
!*/
#![forbid(unsafe_code)]

use crayon::utils::handle::{HandleIndex, HandleLike};
use crayon::utils::object_pool::ObjectPool;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

static FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct MyHandle {
    indices: [HandleIndex; 2],
    version: HandleIndex,
}

impl HandleLike for MyHandle {
    fn new(index: HandleIndex, version: HandleIndex) -> Self {
        MyHandle {
            indices: [index, index],
            version,
        }
    }

    fn index(&self) -> HandleIndex {
        if dbg!(FLAG.fetch_xor(true, Ordering::Relaxed)) {
            self.indices[1]
        } else {
            self.indices[0]
        }
    }

    fn version(&self) -> HandleIndex {
        self.version
    }
}

impl MyHandle {
    fn with_indices(indices: [HandleIndex; 2], version: HandleIndex) -> Self {
        MyHandle { indices, version }
    }
}

fn main() {
    let mut pool = ObjectPool::new();
    let real_handle: MyHandle = pool.create(123);
    let fake_handle =
        MyHandle::with_indices([real_handle.index(), 12345678], real_handle.version());

    // Segfault with OOB, accessing`pool.entries[12345678]` without boundary checking
    dbg!(pool.get(fake_handle));

    // The bug can be similarly triggered in all other methods of `ObjectPool`
    // that call `handle.index()` in an unsafe block.
}

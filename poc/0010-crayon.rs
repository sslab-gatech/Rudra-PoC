/*!
```rudra-poc
[target]
crate = "crayon"
version = "0.7.1"

[test]
analyzers = ["manual", "UnsafeDestructor"]

[report]
title = "Misbehaving `HandleLike` implementation can lead to memory safety violation"
description = """
Unsafe code in `ObjectPool` has time-of-check to time-of-use (TOCTOU) bug that can eventually lead to a memory safety violation. \
`ObjectPool` and `HandlePool` implicitly assumes that `HandleLike` trait methods are pure, i.e., they always return the same value. \
However, this assumption is unsound since `HandleLike` is a safe, public trait that allows a custom implementation."""
code_snippets = [
    "https://github.com/shawnscode/crayon/blob/48d4e879996e2502e0faaf36e4dbcebfca9961b0/src/utils/handle.rs#L90-L94",
    "https://github.com/shawnscode/crayon/blob/48d4e879996e2502e0faaf36e4dbcebfca9961b0/src/utils/object_pool.rs#L48-L66",
    "https://github.com/shawnscode/crayon/blob/48d4e879996e2502e0faaf36e4dbcebfca9961b0/src/utils/object_pool.rs#L164-L174",
]
patched = []
informational = "unsound"
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

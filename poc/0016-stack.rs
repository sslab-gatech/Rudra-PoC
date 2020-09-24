/*!
```crux-poc
[target]
crate = "stack"
version = "0.3.0"

[test]
analyzers = ["UnsafeDestructor"]
cargo_flags = ["--release"]

[report]
title = "Missing check in ArrayVec leads to out-of-bounds write."
description = """
ArrayVec::insert allows insertion of an element into the array object into the
specified index. Due to a missing check on the upperbound of this index, it is
possible to write out of bounds.
"""
code_snippets = []
patched = []
issue_url = "https://github.com/arcnmx/stack-rs/issues/4"
issue_date = 2020-09-24
rustsec_url = "https://github.com/RustSec/advisory-db/pull/392"
```
!*/
#![forbid(unsafe_code)]

use stack::{ArrayVec, Vector, Array};

fn main() {
    // 1. `Vector::insert` is missing an upper bounds-check on its index
    //    allowing for arbitrary memory writes.
    //
    // In debug mode this will panic with integer overflow. In release mode it
    // segfaults the program when it writes junk into the heap.
    //
    // https://github.com/arcnmx/stack-rs/blob/76cc1855a3ce966182bcf6fb2dc6a1d765cb5138/src/vector.rs#L51-L60
    let mut vec : ArrayVec<[u8; 2]> = ArrayVec::with_capacity(2);
    vec.insert(5, 0x41);

    // 2. The `array` in `ArrayVec` uses `std::mem::uninitialized`. While this
    //    isn't a big concern, seeing as only a few types are allowed to inhabit
    //    ArrayVec<T>, this should be replaced. See https://github.com/servo/rust-smallvec/issues/126
}

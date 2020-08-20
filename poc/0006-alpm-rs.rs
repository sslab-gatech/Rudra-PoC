/*!
```crux-poc
[target]
crate = "alpm-rs"
version = "0.1.24"

[test]
analyzers = ["UnsafeDestructor"]

[report]
title = "StrcCtx allows safe Rust code to deallocate a memory region"
description = """
`StrcCtx` allows safe Rust code to deallocate a memory region when it is created without using `StrcCtx::new`.
This can introduce memory safety issues such as double-free and use-after-free to client programs."""
code_snippets = ["https://github.com/pigeonhands/rust-arch/blob/8458c22a161cb8996659fd124de49972f8164712/alpm-rs/src/macros.rs#L18-L38"]
patched = []
informational = "unsound"
```
!*/
#![forbid(unsafe_code)]

use alpm_rs::macros::StrcCtx;

fn main() {
    let mut v1: Vec<i8> = vec![1, 2, 3, 0];
    let _ = StrcCtx {
        ptr: v1.as_mut_ptr(),
    };

    // use-after-free in v1
    // v1 and v2 are backed by the same buffer
    let v2: Vec<i8> = vec![4, 5, 6, 0];

    let measure1 = v2[0];
    v1[0] = 123;
    let measure2 = v2[0];

    assert_eq!(measure1, measure2);
}

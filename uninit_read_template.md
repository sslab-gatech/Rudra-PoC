# ((Issue Title))

Hello fellow Rustacean,
we (Rust group @sslab-gatech) found a memory-safety/soundness issue in this crate while scanning Rust code on crates.io for potential vulnerabilities.

## Issue Description

`foo()` method creates an uninitialized buffer and passes it to user-provided `Read` implementation. This is unsound, because it allows safe Rust code to exhibit an undefined behavior (read from uninitialized memory).

[This part](https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read) from the `Read` trait documentation explains the issue:

> It is your responsibility to make sure that `buf` is initialized before calling `read`. Calling read with an uninitialized `buf` (of the kind one obtains via `MaybeUninit<T>`) is not safe, and can lead to undefined behavior.

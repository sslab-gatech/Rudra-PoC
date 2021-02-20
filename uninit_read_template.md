# ((Issue Title))

Hello,
we (Rust group @sslab-gatech) found a memory-safety/soundness issue in this crate while scanning Rust code on crates.io for potential vulnerabilities.

## Issue Description

`foo()` method creates an uninitialized buffer and passes it to user-provided `Read` implementation. This is unsound, because it allows safe Rust code to exhibit an undefined behavior (read from uninitialized memory).

In case a user-provided `Read` reads from the given buffer, uninitialized buffer can make safe Rust code to cause memory safety errors by miscompilation. Uninitialized values are lowered to LLVM as [`llvm::UndefValue`](https://llvm.org/doxygen/classllvm_1_1UndefValue.html#details) which may take different random values for each read. Propagation of `UndefValue` can quickly cause safe Rust code to exhibit undefined behavior.

[This part](https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read) from the `Read` trait documentation explains the issue:

> It is your responsibility to make sure that `buf` is initialized before calling `read`. Calling read with an uninitialized `buf` (of the kind one obtains via `MaybeUninit<T>`) is not safe, and can lead to undefined behavior.

## How to fix the issue?

The Naive & safe way to fix the issue is to always zero-initialize a buffer before lending it to a user-provided `Read` implementation. Note that this approach will add runtime performance overhead of zero-initializing the buffer.

As of Feb 2021, there is not yet an ideal fix that works with no performance overhead. Below are links to relevant discussions & suggestions for the fix.

* [Rust RFC 2930](https://github.com/rust-lang/rfcs/blob/master/text/2930-read-buf.md#summary)
* [Discussion in Rust Internals Forum](https://internals.rust-lang.org/t/uninitialized-memory/1652)

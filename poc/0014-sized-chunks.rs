/*!
```rudra-poc
[target]
crate = "sized-chunks"
version = "0.6.2"

[[target.peer]]
crate = "typenum"
version = "1.11.2"

[test]
analyzers = ["manual"]
cargo_flags = ["--release"]

[report]
title = "Multiple soundness issues in Chunk and InlineArray"
description = """
Chunk:

* Array size is not checked when constructed with `unit()` and `pair()`.
* Array size is not checked when constructed with `From<InlineArray<A, T>>`.
* `Clone` and `insert_from` are not panic-safe; A panicking iterator causes memory safety issues with them.

InlineArray:

* Generates unaligned references for types with a large alignment requirement."""
code_snippets = []
patched = []
issue_url = "https://github.com/bodil/sized-chunks/issues/11"
issue_date = 2020-09-06
rustsec_url = "https://github.com/RustSec/advisory-db/pull/381"
rustsec_id = "RUSTSEC-2020-0041"
```
!*/
#![forbid(unsafe_code)]

mod boilerplate;

use sized_chunks::{Chunk, InlineArray};
use typenum::*;

#[repr(align(256))]
struct LargeAlign(u8);

struct DropDetector(u32);

impl DropDetector {
    fn new(num: u32) -> Self {
        println!("Creating {}", num);
        DropDetector(num)
    }
}

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

impl Clone for DropDetector {
    fn clone(&self) -> Self {
        if self.0 == 42 {
            panic!("panic on clone")
        }
        DropDetector::new(self.0)
    }
}

struct PanickingIterator {
    current: u32,
    panic_at: u32,
    len: usize,
}

impl Iterator for PanickingIterator {
    type Item = DropDetector;

    fn next(&mut self) -> Option<Self::Item> {
        let num = self.current;

        if num == self.panic_at {
            panic!("panicking index")
        }

        self.current += 1;
        Some(DropDetector::new(num))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl ExactSizeIterator for PanickingIterator {}

fn main() {
    boilerplate::init();

    // Some of these cases will panic earlier than assert in debug build due to overflow detection,
    // but they still have the same error

    // https://github.com/bodil/sized-chunks/blob/40aa74b824688a4d4b1e1c65a50c679abb58b41e/src/sized_chunk/mod.rs#L153-L177
    boilerplate::test_case(
        "1. Array size is not checked when constructed with `unit()` and `pair()`.",
        || {
            let _ = Chunk::<usize, U0>::unit(123);
            let mut chunk = Chunk::<usize, U0>::pair(123, 456);

            // Moreover, we can push more elements because `is_full` is implemented as `len != capacity`
            chunk.push_back(789);

            println!("len: {}", chunk.len());
            assert!(chunk.len() <= U0::USIZE);
        },
    );

    // https://github.com/bodil/sized-chunks/blob/40aa74b824688a4d4b1e1c65a50c679abb58b41e/src/sized_chunk/mod.rs#L815-L829
    boilerplate::test_case(
        "2. Array size is not checked when constructed with `From<InlineArray<A, T>>`",
        || {
            let mut from = InlineArray::<u8, [u8; 256]>::new();
            from.push(1);
            from.push(2);
            from.push(3);
            from.push(4);
            from.push(5);

            let to = Chunk::<u8, U0>::from(from);
            println!("len: {}", to.len());
            assert!(to.len() <= U0::USIZE);
        },
    );

    // https://github.com/bodil/sized-chunks/blob/40aa74b824688a4d4b1e1c65a50c679abb58b41e/src/sized_chunk/mod.rs#L120-L134
    boilerplate::test_case("3-1. `Chunk::clone()` is not panic-safe", || {
        let mut chunk = Chunk::<DropDetector, U3>::new();
        chunk.push_back(DropDetector::new(42));
        chunk.push_back(DropDetector::new(43));

        // observe the difference between creating and dropping log
        // uninitialized memory is dropped while unwinding
        println!("=> Dropping uninitialized memory");
        let _ = chunk.clone();
    });

    // https://github.com/bodil/sized-chunks/blob/40aa74b824688a4d4b1e1c65a50c679abb58b41e/src/sized_chunk/mod.rs#L564-L617
    boilerplate::test_case("3-2. `Chunk::insert_from()` is not panic-safe", || {
        let mut chunk = Chunk::<DropDetector, U5>::new();
        chunk.push_back(DropDetector::new(1));
        chunk.push_back(DropDetector::new(2));
        chunk.push_back(DropDetector::new(3));

        println!("=> Double-free of `DropDetector(2)`");
        chunk.insert_from(
            1,
            PanickingIterator {
                current: 1,
                panic_at: 1,
                len: 1,
            },
        );
    });

    boilerplate::test_case("4. `InlineArray` generates unaligned references for types with a large alignment requirement.", || {
        let mut arr = InlineArray::<LargeAlign, [usize; 256]>::new();
        arr.push(LargeAlign(0));

        boilerplate::assert_aligned(arr.get(0).unwrap());
    });

    // Other issues that should be fixed but probably minor to include in the advisory:

    // https://github.com/bodil/sized-chunks/blob/40aa74b824688a4d4b1e1c65a50c679abb58b41e/src/sized_chunk/mod.rs#L564-L617
    // `insert_from` relies on the behavioral correctness of `ExactSizeIterator`.
    // However, `ExactSizeIterator` is a safe trait, which has the same safety guarantee with `size_hint()`.
    // Programs should not assume that they will yield a correct value in unsafe code.
    // From Rust std doc: "An incorrect implementation of `size_hint()` should not lead to memory safety violations."
    //
    // Applying `take(insert_size)` and adjusting `left` and `right` field based on the number of items that are actually moved
    // (instead of using `insert_size`) will fix the problem.

    // https://github.com/bodil/sized-chunks/blob/40aa74b824688a4d4b1e1c65a50c679abb58b41e/src/inline_array/mod.rs#L167
    // This states an actual contract, so it should be `assert!()` instead of `debug_assert!()`
    // From Rust std doc: "Replacing `assert!` with `debug_assert!` is thus only encouraged after thorough profiling, and more importantly, only in safe code!"
}

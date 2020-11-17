/*!
```crux-poc
[target]
crate = "unicycle"
version = "0.6.3"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "PinSlab<T> and Unordered<T, S> need proper bounds on their Send/Sync traits"
description = """
issue description"""
code_snippets = ["https://github.com/udoprog/unicycle/blob/f5a283826ef91e5a07f4b43fea9ec8ae281f84cb/src/pin_slab.rs#L43-L44", "https://github.com/udoprog/unicycle/blob/f5a283826ef91e5a07f4b43fea9ec8ae281f84cb/src/lib.rs#L380-L381"]
patched = []
informational = "unsound"
issue_url = "https://github.com/udoprog/unicycle/issues/8"
issue_date = 2020-11-15
```
!*/
#![forbid(unsafe_code)]

use unicycle::pin_slab::PinSlab;

use std::cell::Cell;
use crossbeam_utils::thread;

// A simple tagged union used to demonstrate problems with data races in Cell.
#[derive(Debug, Clone, Copy)]
enum RefOrInt { Ref(&'static u64), Int(u64) }
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));

    let mut slab = PinSlab::new();
    slab.insert(&cell);

    thread::scope(|s| {
        s.spawn(move |_| {
            loop {
                let smuggled_cell = slab.get(0).unwrap();
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&SOME_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            if let RefOrInt::Ref(addr) = cell.get() {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u64 == &SOME_INT as *const u64 { continue; }

                // Due to the data race, obtaining Ref(0xdeadbeef) is possible
                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}
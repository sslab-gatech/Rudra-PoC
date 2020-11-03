/*!
```crux-poc
[target]
crate = "convec"
version = "2.0.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "convec::ConVec lacks a bound on its Sync trait allowing for data races"
description = """
`convec::ConVec` implements `Sync` for any type.
This allows objects like `Cell` that doesn't implement `Sync` to be shared across threads leading to undefined behavior.
"""
code_snippets = ["https://github.com/krl/convec/blob/1591dcdda05fdbc5483333411c0c7ac7e16f61c7/src/convec.rs#L16"]
patched = []
informational = "unsound"
```
!*/
#![forbid(unsafe_code)]

use std::cell::Cell;

use crossbeam_utils::thread;
use convec::*;

static SOME_INT: u128 = 0x41414141;

fn main() {
    // A simple tagged union used to demonstrate the problems with data races
    // in Cell. Cell is designed for single threads and has no synchronization
    // methods. Thus if it is allowed to be used simaltaneously by two threads,
    // it is possible to race its interior mutability methods to derference an
    // arbitrary pointer.
    #[derive(Debug, Clone, Copy)]
    enum RefOrInt<'a> {
        Ref(&'a u128),
        Int(u128),
    }

    let cell_array = [Cell::new(RefOrInt::Ref(&SOME_INT))];
    thread::scope(|s| {
        let av1: AoVec<&[Cell<RefOrInt>; 1]> = AoVec::new();
        let av2: AoVec<&[Cell<RefOrInt>; 1]> = AoVec::new();
        av1.push(&cell_array);
        av2.push(&cell_array);

        let child = s.spawn(move |_| {
            loop {
                let cell_array = av2.get(0).unwrap();
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                cell_array[0].set(RefOrInt::Ref(&SOME_INT));
                cell_array[0].set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            let main_thread_cell = av1.get(0).unwrap()[0].clone().into_inner();
            if let RefOrInt::Ref(addr) = main_thread_cell {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u128 == &SOME_INT as *const u128 {
                    continue;
                }

                // Due to the data race, obtaining Ref(0xdeadbeef) is possible
                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

/*!
```rudra-poc
[target]
crate = "beef"
version = "0.4.4"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "beef::Cow lacks a Sync bound on its Send trait allowing for data races"
description = """
`beef::Cow` implements `Send` for any type that implements `Send`. However,
it allows multiple read-only references to the underlying data creating a read
data race. This allows objects like `RefCell` that implement `Send` but not
`Sync` to be shared across threads leading to undefined behavior.
"""
code_snippets = ["https://github.com/maciejhirsz/beef/blob/0b4685143e680749991c295836d8d09565fd6814/src/generic.rs#L531"]
patched = []
informational = "unsound"
issue_url = "https://github.com/maciejhirsz/beef/issues/37"
issue_date = 2020-10-28
```
!*/
#![forbid(unsafe_code)]

use crossbeam_utils::thread;
use std::cell::Cell;

use beef::Cow;

static SOME_INT: u64 = 123;

fn main() {
    // A simple tagged union used to demonstrate the problems with data races
    // in Cell. Cell is designed for single threads and has no synchronization
    // methods. Thus if it is allowed to be used simultaneously by two threads,
    // it is possible to race its interior mutability methods to dereference an
    // arbitrary pointer.
    #[derive(Debug, Clone, Copy)]
    enum RefOrInt<'a> {
        Ref(&'a u64),
        Int(u64),
    }

    let cell_array = [Cell::new(RefOrInt::Ref(&SOME_INT))];
    thread::scope(|s| {
        let cow1: Cow<[Cell<RefOrInt>]> = Cow::borrowed(cell_array.as_ref());
        let cow2: Cow<[Cell<RefOrInt>]> = cow1.clone();

        let child = s.spawn(move |_| {
            // We've now smuggled the cell from above into this thread.
            let smuggled_cell = cow2.unwrap_borrowed();
            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell[0].set(RefOrInt::Ref(&SOME_INT));
                smuggled_cell[0].set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            let main_thread_cell = (*cow1)[0].clone().into_inner();
            if let RefOrInt::Ref(addr) = main_thread_cell {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u64 == &SOME_INT as *const u64 {
                    continue;
                }

                // Due to the data race, obtaining Ref(0xdeadbeef) is possible
                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

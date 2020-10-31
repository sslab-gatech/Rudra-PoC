/*!
```crux-poc
[target]
crate = "atomic-option"
version = "0.1.2"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "atomic-option allows using non-Sync types across threads leading to data races"
description = """
atomic_option::AtomicOption implements the Sync trait without a bound on its
generic type `T`. This allows sending across non-Sync objects across threads
such as `Cell` leading to data-races.
"""
code_snippets = ["https://github.com/reem/rust-atomic-option/blob/a1c645731b5cc4e01924dd10dadea222c425d576/src/lib.rs#L23"]
patched = []
informational = "unsound"
```
!*/
#![forbid(unsafe_code)]

use atomic_option::AtomicOption;

use crossbeam_utils::thread;
use std::{cell::Cell, sync::atomic::Ordering};

static SOME_INT: u64 = 123;

fn main() {
    #[derive(Debug, Clone, Copy)]
    enum RefOrInt<'a> {
        Ref(&'a u64),
        Int(u64),
    }
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));

    let atomic_opt = AtomicOption::new(Box::new(&cell));
    let ref_to_atomic_opt = &atomic_opt;

    thread::scope(|s| {
        s.spawn(move |_| {
            let cell_in_thread = *(ref_to_atomic_opt.take(Ordering::Relaxed).unwrap());
            println!("Thread - {:p} - {:?}", cell_in_thread, cell_in_thread);

            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                cell_in_thread.set(RefOrInt::Ref(&SOME_INT));
                cell_in_thread.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        println!("Main - {:p} - {:?}", &cell, cell);
        loop {
            if let RefOrInt::Ref(addr) = cell.get() {
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

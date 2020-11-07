/*!
```crux-poc
[target]
crate = "lock_api"
version = "0.4.1"

[[target.peer]]
crate = "parking_lot"
version = "0.11.0"
features = ["send_guard"]
[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "issue title"
description = """
issue description"""
code_snippets = []
patched = []
informational = "unsound"
```
!*/
#![forbid(unsafe_code)]

use parking_lot::{Mutex, MutexGuard};
use std::cell::Cell;

use crossbeam_utils::thread;

static SOME_INT: u64 = 123;

fn main() {
    #[derive(Debug, Clone, Copy)]
    enum RefOrInt<'a> {
        Ref(&'a u64),
        Int(u64),
    }

    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let mutex = Mutex::new(&cell);

    thread::scope(|s| {
        let guard = mutex.lock();
        // MappedMutexGuard that just returns the whole object as a "component".
        let mapped_guard = MutexGuard::map(guard, |x| x);

        let child = s.spawn(move |_| {
            let smuggled_cell = *mapped_guard;

            println!("Thread - {:p} - {:?}", smuggled_cell, smuggled_cell);
            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&SOME_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
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

/*!
```crux-poc
[target]
crate = "futures-intrusive"
version = "0.3.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "futures_intrusive's GenericMutexGuard lacks !Sync allowing for data races"
description = """
The GenericMutexGuard, meant to act as an RII lock acquisition guard is
automatically marked as Sync by Rust. However, it allows multiple threads to
read out a non-Sync object `T` so long as it implements Send.

This opens up the possibility for data-races if a GenericMutexGuard is sent
across threads.
"""
code_snippets = []
patched = []
informational = "unsound"
issue_url = "https://github.com/Matthias247/futures-intrusive/issues/53"
issue_date = "2020-10-31"
```
!*/
#![forbid(unsafe_code)]

use futures_intrusive::sync::{GenericMutexGuard, Mutex};

use crossbeam_utils::thread;
use std::cell::Cell;

static SOME_INT: u64 = 123;

fn main() {
    #[derive(Debug, Clone, Copy)]
    enum RefOrInt<'a> {
        Ref(&'a u64),
        Int(u64),
    }
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));

    let futures_mutex: Mutex<Cell<_>> = Mutex::new(cell, false);
    let mutex_guard: GenericMutexGuard<_, Cell<_>> = futures_mutex.try_lock().unwrap();

    thread::scope(|s| {
        let guard_ref = &mutex_guard;
        let child = s.spawn(move |_| {
            let smuggled = &(**guard_ref);

            println!("In the thread: {:p} {:?}", smuggled, *smuggled);
            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled.set(RefOrInt::Ref(&SOME_INT));
                smuggled.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        println!("In main: {:p} {:?}", &(*mutex_guard), *mutex_guard);
        loop {
            if let RefOrInt::Ref(addr) = mutex_guard.get() {
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

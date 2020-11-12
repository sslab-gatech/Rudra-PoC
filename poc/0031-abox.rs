/*!
```crux-poc
[target]
crate = "abox"
version = "0.4.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "abox's AtomicBox lacks bounds on its Send/Sync traits allowing data races"
description = """
issue description"""
code_snippets = ["https://github.com/SonicFrog/abox/blob/5abe75222bc49af6b62ea37f87d7be0c56973310/src/lib.rs#L92-L93"]
patched = []
informational = "unsound"
issue_url = "https://github.com/SonicFrog/abox/issues/1"
issue_date = 2020-11-10
```
!*/
#![forbid(unsafe_code)]

use abox::AtomicBox;

use std::cell::Cell;
use crossbeam_utils::thread;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> { Ref(&'a u64), Int(u64) }
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let atomic_box = AtomicBox::new(&cell);

    thread::scope(|s| {
        s.spawn(move |_| {
            let smuggled_cell = atomic_box.get();

            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&SOME_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            }
        });

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

/*!
```rudra-poc
[target]
crate = "late-static"
version = "0.3.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "LateStatic lacks a Sync bound on its contained type allowing for data races"
description = """
"""
code_snippets = ["https://github.com/Richard-W/late-static/blob/3b72ba58df151a821405551358edbe0b015545f3/src/lib.rs#L29-L30"]
patched = []
informational = "unsound"
issue_url = "https://github.com/Richard-W/late-static/issues/1"
issue_date = 2020-11-10
```
!*/

use late_static::LateStatic;

use std::cell::Cell;
use std::thread;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

static STATIC_CELL: LateStatic<Cell<RefOrInt>> = LateStatic::new();

fn main() {
    unsafe {
        LateStatic::assign(&STATIC_CELL, Cell::new(RefOrInt::Ref(&SOME_INT)));
    }

    thread::spawn(move || {
        loop {
            // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
            STATIC_CELL.set(RefOrInt::Ref(&SOME_INT));
            STATIC_CELL.set(RefOrInt::Int(0xdeadbeef));
        }
    });

    loop {
        if let RefOrInt::Ref(addr) = STATIC_CELL.get() {
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
}

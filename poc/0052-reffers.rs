/*!
```crux-poc
[target]
crate = "reffers"
version = "0.6.0"

[test]
analyzers = []

[report]
title = "Unsound: can make `ARefss` contain a !Send, !Sync object"
description = """
issue description"""
code_snippets = ["https://docs.rs/reffers/0.6.0/src/reffers/aref.rs.html#206-208"]
patched = []
informational = "unsound"
issue_url = "https://github.com/diwic/reffers-rs/issues/7"
issue_date = 2020-12-01
```
!*/
#![forbid(unsafe_code)]

use std::cell::Cell;
use std::sync::Arc;

use reffers::aref::ARefss;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}

static X: u64 = 0;
fn main() {
    let arc_0 = Arc::new(ARefss::new(Arc::new(0)).map(|_| {
        // New item is totally unrelated to the previously stored item.
        // New item is allowed to be !Sync, !Send.
        Box::leak(Box::new(Cell::new(RefOrInt::Ref(&X))))
        // Box::leak(Box::new(std::rc::Rc::new(0)))
    }));
    let arc_child = Arc::clone(&arc_0);

    std::thread::spawn(move || {
        let arc_child = arc_child;

        let smuggled_cell = arc_child.as_ref();
        loop {
            smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            smuggled_cell.set(RefOrInt::Ref(&X));
        }
    });

    loop {
        if let RefOrInt::Ref(addr) = arc_0.get() {
            if addr as *const _ as usize == 0xdeadbeef {
                // Due to the data race, obtaining Ref(0xdeadbeef) is possible
                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    }
}
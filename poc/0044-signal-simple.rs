/*!
```rudra-poc
[target]
crate = "signal-simple"
version = "0.1.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "SyncChannel<T> lacks a Send bound on its Send and Sync traits"
description = """
issue description"""
code_snippets = ["https://github.com/kitsuneninetails/signal-rust/blob/2d671bde04b87c741134e5e1a5cd491ae54768e6/src/channel.rs#L58-L59"]
patched = []
informational = "unsound"
issue_url = "https://github.com/kitsuneninetails/signal-rust/issues/2"
issue_date = 2020-11-15
```
!*/
#![forbid(unsafe_code)]

use signal_simple::channel::SyncChannel;

use crossbeam_utils::thread;
use std::cell::Cell;

// A simple tagged union used to demonstrate problems with data races in Cell.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));

    let channel = SyncChannel::new();
    channel.send(&cell);

    thread::scope(|s| {
        s.spawn(|_| {
            let smuggled_cell = channel.recv().unwrap();
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

                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

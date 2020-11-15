/*!
```crux-poc
[target]
crate = "may_queue"
version = "0.1.7"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "may's Queue object lacks a Send bound on its Send/Sync traits allowing for data races"
description = """
The Queue object in Send implements the Send trait unconditionally, potentially
allowing for data races across threads.
"""
code_snippets = ["https://github.com/Xudong-Huang/may/blob/0abc40e67034b297614fd01517b46c224b8f79eb/may_queue/src/mpsc_list_v1.rs#L150-L151"]
patched = []
informational = "unsound"
issue_url = "https://github.com/Xudong-Huang/may/issues/88"
issue_date = 2020-11-10
```
!*/
#![forbid(unsafe_code)]

use may_queue::mpsc_list::Queue;

use std::cell::Cell;
use crossbeam_utils::thread;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let queue = Queue::new();
    queue.push(&cell);

    thread::scope(|s| {
        s.spawn(move |_| {
            let smuggled_cell = queue.pop().unwrap();

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
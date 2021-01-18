/*!
```rudra-poc
[target]
crate = "multiqueue2"
version = "0.1.6"

[[target.peer]]
crate = "futures"
version = "0.1.27"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/abbychau/multiqueue2/issues/10"
issue_date = 2020-12-19
unique_bugs = 2
additional_send_sync_violations = 2
```
!*/
#![forbid(unsafe_code)]
use std::cell::Cell;
use std::sync::Arc;
use std::thread;
// futures = "0.1.27"
use futures::{Future, Sink, Stream};

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static X: u64 = 0;

use multiqueue2::mpmc_fut_queue;

fn main() {
    let (tx, rx) = mpmc_fut_queue(16);
    let cell = Arc::new(Cell::new(RefOrInt::Int(0xdeadbeef)));
    let sent = tx.send(Arc::clone(&cell));

    thread::spawn(move || {
        let mut rx = rx.wait();

        // parent thread sent us an object that is not `Send`!
        let smuggled_cell = rx.next().unwrap().unwrap();

        loop {
            smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            smuggled_cell.set(RefOrInt::Ref(&X))
        }
    });
    sent.wait().unwrap();

    loop {
        if let RefOrInt::Ref(addr) = cell.get() {
            if addr as *const _ as usize != 0xdeadbeef {
                continue;
            }
            // Due to the data race, obtaining Ref(0xdeadbeef) is possible
            println!("Pointer is now: {:p}", addr);
            println!("Dereferencing addr will now segfault: {}", *addr);
        }
    }
}

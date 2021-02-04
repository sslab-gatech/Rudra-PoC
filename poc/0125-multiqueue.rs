/*!
```rudra-poc
[target]
crate = "multiqueue"
version = "0.3.2"

[[target.peer]]
crate = "futures"
version = "0.1.27"

[report]
issue_url = "https://github.com/schets/multiqueue/issues/31"
issue_date = 2020-12-25
rustsec_url = "https://github.com/RustSec/advisory-db/pull/744"
rustsec_id = "RUSTSEC-2020-0143"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 4
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

use multiqueue::mpmc_fut_queue;

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

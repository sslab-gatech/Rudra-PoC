/*!
```rudra-poc
[target]
crate = "ticketed_lock"
version = "0.2.0"

[[target.peer]]
crate = "futures"
version = "0.1.14"

[report]
issue_url = "https://github.com/kvark/ticketed_lock/issues/7"
issue_date = 2020-11-17
rustsec_url = "https://github.com/RustSec/advisory-db/pull/678"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]

use ticketed_lock::TicketedLock;

use futures::Future;
use std::{rc::Rc, thread};

fn main() {
    let rc = Rc::new(());
    let rc_clone = rc.clone();

    let mut lock = TicketedLock::new(rc_clone);

    let read_ticket = lock.read();
    thread::spawn(move || {
        let smuggled_rc = read_ticket.wait().unwrap();

        println!("Thread: {:p}", *smuggled_rc);
        // Race the refcount with the main thread.
        for _ in 0..100_000_000 {
            smuggled_rc.clone();
        }
    });

    println!("Main:   {:p}", rc);
    for _ in 0..100_000_000 {
        rc.clone();
    }
}

/*!
```rudra-poc
[target]
crate = "tiny_future"
version = "0.3.2"

[report]
issue_url = "https://github.com/KizzyCode/tiny_future/issues/1"
issue_date = 2020-12-08
rustsec_url = "https://github.com/RustSec/advisory-db/pull/675"
rustsec_id = "RUSTSEC-2020-0118"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/lib.rs:165:1: 165:43", "src/lib.rs:166:1: 166:43"]
sendsync_details = ["NeedSend", "NeedBoth"]
```
!*/
#![forbid(unsafe_code)]

use std::{rc::Rc, thread};
use tiny_future::Future;

fn main() {
    let rc = Rc::new(());
    let rc_clone = rc.clone();

    let f = Future::with_state(());
    f.set(rc_clone);

    thread::spawn(move || {
        let smuggled_rc = f.get().unwrap();

        println!("Thread: {:p}", smuggled_rc);
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

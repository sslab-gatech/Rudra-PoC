/*!
```rudra-poc
[target]
crate = "slock"
version = "0.1.2"

[[target.peer]]
crate = "futures"
version = "0.3.8"
features = ["thread-pool"]

[report]
issue_url = "https://github.com/BrokenLamp/slock-rs/issues/2"
issue_date = 2020-11-17
rustsec_url = "https://github.com/RustSec/advisory-db/pull/652"
rustsec_id = "RUSTSEC-2020-0135"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/lib.rs:268:1: 268:36", "src/lib.rs:269:1: 269:36"]
```
!*/
#![forbid(unsafe_code)]

use slock::Slock;

use std::rc::Rc;
use futures::executor::ThreadPool;

fn main() {
    let rc = Rc::new(());
    let lock = Slock::new(rc);
    let another_lock = lock.split();

    let fut1 = async move {
        let rc = lock.get_clone().await;
        println!("Future 1 - {:p}", rc);

        // Race the un-synchronized refcount in the RCs.
        for _ in 0..100_000_000 {
            rc.clone();
        }
    };
    let fut2 = async move {
        let rc = another_lock.get_clone().await;
        println!("Future 2 - {:p}", rc);

        for _ in 0..100_000_000 {
            rc.clone();
        }
    };

    let mut pool = ThreadPool::new().unwrap();
    pool.spawn_ok(fut1);
    pool.spawn_ok(fut2);
    // Give the pool time to complete its tasks.
    loop {}
}

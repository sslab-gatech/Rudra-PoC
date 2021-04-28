/*!
```rudra-poc
[target]
crate = "noise_search"
version = "0.7.0"

[report]
issue_url = "https://github.com/pipedown/noise/issues/72"
issue_date = 2020-12-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/731"
rustsec_id = "RUSTSEC-2020-0141"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/index.rs:419:1: 419:41", "src/index.rs:420:1: 420:41"]
sendsync_details = ["NeedSend", "NeedSync"]
```
!*/
#![forbid(unsafe_code)]

use noise_search::index::MvccRwLock;

use std::rc::Rc;

fn main() {
    let rc = Rc::new(42);

    let lock = MvccRwLock::new(rc.clone());
    std::thread::spawn(move || {
        let smuggled_rc = lock.read();

        println!("Thread: {:p}", *smuggled_rc);
        for _ in 0..100_000_000 {
            (*smuggled_rc).clone();
        }
    });

    println!("Main:   {:p}", rc);
    for _ in 0..100_000_000 {
        rc.clone();
    }
}

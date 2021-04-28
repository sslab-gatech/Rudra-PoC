/*!
```rudra-poc
[target]
crate = "aovec"
version = "1.1.0"

[report]
issue_date = 2020-12-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/528"
rustsec_id = "RUSTSEC-2020-0099"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/lib.rs:17:1: 17:36", "src/lib.rs:16:1: 16:36"]
sendsync_details = ["NeedSend", "NeedBoth"]
```
!*/
#![forbid(unsafe_code)]

use aovec::Aovec;
use std::rc::Rc;

fn main() {
    let vec = Aovec::new(1);

    let rc = Rc::new(42);
    vec.push(rc.clone());

    std::thread::spawn(move || {
        println!("Thread: {:p}", vec[0]);
        for _ in 0..100_000_000 {
            vec[0].clone();
        }
    });

    println!("Main:   {:p}", rc);
    for _ in 0..100_000_000 {
        rc.clone();
    }
}

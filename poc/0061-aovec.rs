/*!
```rudra-poc
[target]
crate = "aovec"
version = "1.1.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
rustsec_url = "https://github.com/RustSec/advisory-db/pull/528"
rustsec_id = "RUSTSEC-2020-0099"
issue_date = 2020-12-10
unique_bugs = 1
additional_send_sync_violations = 1
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

/*!
```rudra-poc
[target]
crate = "thex"
version = "0.1.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_date = 2020-12-08
rustsec_url = "https://github.com/RustSec/advisory-db/pull/523"
rustsec_id = "RUSTSEC-2020-0090"
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

use std::rc::Rc;
use thex::Thex;

fn main() {
    let rc = Rc::new(());
    let rc_clone = rc.clone();

    let thex = Thex::new(rc_clone);
    std::thread::spawn(move || {
        let smuggled_rc = thex.shared();

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

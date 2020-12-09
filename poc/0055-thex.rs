/*!
```rudra-poc
[target]
crate = "thex"
version = "0.1.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
issue_date = 2020-12-08
rustsec_url = "https://github.com/RustSec/advisory-db/pull/523"
```
!*/
#![forbid(unsafe_code)]

use thex::Thex;
use std::rc::Rc;

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
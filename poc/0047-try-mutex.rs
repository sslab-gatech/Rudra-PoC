/*!
```rudra-poc
[target]
crate = "try-mutex"
version = "0.2.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[report]
issue_url = "https://github.com/mpdn/try-mutex/issues/2"
issue_date = 2020-11-17
rustsec_url = "https://github.com/RustSec/advisory-db/pull/517"
rustsec_id = "RUSTSEC-2020-0087"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/lib.rs:125:1: 125:40", "src/lib.rs:126:1: 126:53"]
```
!*/
#![forbid(unsafe_code)]

use try_mutex::TryMutex;

use crossbeam_utils::thread;
use std::rc::Rc;

fn main() {
    let rc = Rc::new(());
    let rc_clone = rc.clone();

    let try_mutex = TryMutex::new(rc_clone);
    thread::scope(|s| {
        s.spawn(|_| {
            let smuggled_rc = try_mutex.try_lock().unwrap();
            println!("RC in thread: {:p}", *smuggled_rc);
        });
    });
    println!("RC in main:   {:p}", rc);
}

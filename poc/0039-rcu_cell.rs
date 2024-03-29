/*!
```rudra-poc
[target]
crate = "rcu_cell"
version = "0.1.8"

[report]
issue_url = "https://github.com/Xudong-Huang/rcu_cell/issues/3"
issue_date = 2020-11-14
rustsec_url = "https://github.com/RustSec/advisory-db/pull/692"
rustsec_id = "RUSTSEC-2020-0131"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/lib.rs:375:1: 375:38", "src/lib.rs:376:1: 376:38"]
```
!*/
#![forbid(unsafe_code)]
use rcu_cell::RcuCell;

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    // `Rc` is neither `Send` nor `Sync`
    let rcu_cell = RcuCell::new(Some(Rc::new(0_i32)));
    let arc_parent = Arc::new(rcu_cell);

    let mut child_threads = vec![];
    for _ in 0..5 {
        let arc_child = Arc::clone(&arc_parent);
        child_threads.push(thread::spawn(move || {
            for _ in 0..1000 {
                let reader = arc_child.as_ref().read();
                // data race on internal `strong_count` of `Rc`
                let _ = Rc::clone(&reader.unwrap());
            }
        }));
    }
    for child in child_threads {
        child.join().expect("failed to join child thread");
    }

    assert_eq!(Rc::strong_count(arc_parent.read().as_ref().unwrap()), 1);
}

/*!
```rudra-poc
[target]
crate = "syncpool"
version = "0.1.5"

[report]
issue_url = "https://github.com/Chopinsky/byte_buffer/issues/2"
issue_date = 2020-11-29
rustsec_url = "https://github.com/RustSec/advisory-db/pull/654"
rustsec_id = "RUSTSEC-2020-0142"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
rudra_report_locations = ["src/bucket.rs:334:1: 334:38"]
```
!*/
#![forbid(unsafe_code)]
use syncpool::prelude::*;

use std::boxed::Box;
use std::rc::Rc;

const N_ITER: usize = 900_000;
const N_THREADS: usize = 6;
fn main() {
    // Non-Send object (to be sent to other threads).
    let rc = Rc::new(0_i32);

    let mut pools = vec![];
    for _ in 0..N_THREADS {
        let mut pool = SyncPool::new();
        let _dummy = pool.get();
        let malicious = Box::new(Rc::clone(&rc));
        pool.put(malicious);
        pools.push(pool);
    }

    let mut children = vec![];
    while let Some(pool) = pools.pop() {
        let c = std::thread::spawn(move || {
            // Moved `pool` to child thread.
            let mut pool = pool;
            let boxed_rc = pool.get();

            for _ in 0..N_ITER {
                // Data race on the internal ref count of `Rc`.
                Rc::clone(boxed_rc.as_ref());
            }
        });
        children.push(c);
    }
    // Join child threads.
    for child in children {
        child.join().unwrap();
    }

    assert_eq!(Rc::strong_count(&rc), 1);
}

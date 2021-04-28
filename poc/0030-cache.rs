/*!
```rudra-poc
[target]
crate = "cache"
version = "0.2.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[report]
issue_url = "https://github.com/krl/cache/issues/1"
issue_date = 2020-11-24
rustsec_url = "https://github.com/RustSec/advisory-db/pull/704"
rustsec_id = "RUSTSEC-2020-0128"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/lib.rs:85:1: 85:36", "src/lib.rs:86:1: 86:36"]
sendsync_details = ["NeedSend", "NeedBoth"]
```
!*/
#![forbid(unsafe_code)]

use std::rc::Rc;

const NUM_CLONES: usize = 1000000;

use cache::*;
use crossbeam_utils::thread;

fn main() {
    let rc = Rc::new(true);
    let cache = Cache::new(1, 4096);
    cache.insert(0, rc.clone());

    // We demonstrate the issue by racing the non-atomic reference counter type `Rc` between two threads.
    thread::scope(|s| {
        let child = s.spawn(|_| {
            // &Rc<bool> sent to another thread
            let smuggled = cache.get::<Rc<bool>>(&0).unwrap();
            for _ in 0..NUM_CLONES {
                std::mem::forget(smuggled.clone());
            }
        });

        // if `child.join().unwrap()` is here, the program succeeds

        for _ in 0..NUM_CLONES {
            std::mem::forget(rc.clone());
        }

        child.join().unwrap();

        // We made NUM_CLONES on both threads plus initial 2 reference.
        // But in reality we'll see that the strong_count varies across every run due to data race.
        assert_eq!(Rc::strong_count(&rc), 2 * NUM_CLONES + 2);
    })
    .unwrap();
}

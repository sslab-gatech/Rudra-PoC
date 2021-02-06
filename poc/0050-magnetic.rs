/*!
```rudra-poc
[target]
crate = "magnetic"
version = "2.0.0"

[report]
issue_url = "https://github.com/johnshaw/magnetic/issues/9"
issue_date = 2020-11-29
rustsec_url = "https://github.com/RustSec/advisory-db/pull/519"
rustsec_id = "RUSTSEC-2020-0088"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 13
rudra_report_locations = [
    "src/spsc.rs:35:1: 35:60",
    "src/spsc.rs:42:1: 42:60",
    "src/mpsc.rs:37:1: 37:60",
    "src/mpsc.rs:44:1: 44:60",
    "src/spmc.rs:37:1: 37:60",
    "src/spmc.rs:45:1: 45:60",
    "src/mpmc.rs:37:1: 37:60",
    "src/mpmc.rs:45:1: 45:60",
    "src/mpsc.rs:45:1: 45:60",
    "src/spmc.rs:38:1: 38:60",
    "src/spmc.rs:46:1: 46:60",
    "src/mpmc.rs:38:1: 38:60",
    "src/mpmc.rs:46:1: 46:60"
]
```
!*/
#![forbid(unsafe_code)]

use magnetic::buffer::dynamic::DynamicBuffer;
use magnetic::mpmc::mpmc_queue;
use magnetic::{Consumer, Producer};

use std::rc::Rc;

fn main() {
    let (p, c) = mpmc_queue(DynamicBuffer::new(32).unwrap());
    const N_ITER: usize = 2_000_000;

    // Send `Consumer` to child thread.
    let t1 = std::thread::spawn(move || {
        for _ in 0..N_ITER {
            // Decrements refcount of `Rc` w/o synchronization
            c.pop().unwrap();
        }
    });

    let original_rc = Rc::new(0_i32);
    for _ in 0..N_ITER {
        // Increments refcount of `Rc` w/o synchronization
        p.push(Rc::clone(&original_rc)).unwrap();
    }
    t1.join().unwrap();

    assert_eq!(Rc::strong_count(&original_rc), 1);
}

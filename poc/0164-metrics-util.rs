/*!
```rudra-poc
[target]
crate = "metrics-util"
version = "0.3.1"

[report]
issue_url = "https://github.com/metrics-rs/metrics/issues/190"
issue_date = 2021-04-07
rustsec_url = "https://github.com/RustSec/advisory-db/pull/936"
rustsec_id = "RUSTSEC-2021-0113"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/bucket.rs:80:1: 80:36", "src/bucket.rs:81:1: 81:36"]
```
!*/
#![forbid(unsafe_code)]
use metrics_util::AtomicBucket;

use std::cell::Cell;
use std::sync::Arc;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}

// The bug triggered in this poc exists in older versions too (v0.4.0-alpha.1),
// but I used the newer version of the crate as the older version fails to build
// on my pc at the time..

static SOME_INT: u64 = 123;
fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let bucket = Arc::new(AtomicBucket::new());
    bucket.push(cell);

    let bucket_clone = bucket.clone();
    std::thread::spawn(move || {
        let bucket = bucket_clone;

        // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
        bucket.data_with(|arr| {
            for cell in arr.iter() {
                loop {
                    cell.set(RefOrInt::Ref(&SOME_INT));
                    cell.set(RefOrInt::Int(0xdeadbeef));
                }
            }
        });
    });

    bucket.data_with(|arr| {
        for cell in arr.iter() {
            loop {
                if let RefOrInt::Ref(addr) = cell.get() {
                    if addr as *const u64 == &SOME_INT as *const u64 {
                        continue;
                    }
                    println!("Pointer is now {:p}", addr);
                    println!("Dereferencing addr will now segfault: {}", *addr);
                }
            }
        }
    });
}

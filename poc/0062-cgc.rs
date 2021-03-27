/*!
```rudra-poc
[target]
crate = "cgc"
version = "0.4.0"

[report]
issue_url = "https://github.com/playXE/cgc/issues/5"
issue_date = 2020-12-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/839"
rustsec_id = "RUSTSEC-2020-0148"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/mem.rs:829:1: 829:34", "src/mem.rs:830:1: 830:34"]

[[bugs]]
analyzer = "Manual"
guide = "SendSyncVariance"
bug_class = "Other"
bug_count = 2
rudra_report_locations = []
```
!*/
#![forbid(unsafe_code)]

use cgc::mem::Ptr;
use std::rc::Rc;

fn wild_sync() {
    // 1. Wild Send and Sync
    let rc = Rc::new(42);
    let ptr = Ptr::new(rc.clone());

    std::thread::spawn(move || {
        let smuggled_rc = ptr.take();

        println!("Thread: {:p}", smuggled_rc);
        for _ in 0..100_000_000 {
            smuggled_rc.clone();
        }
    });

    println!("Main:   {:p}", rc);
    for _ in 0..100_000_000 {
        rc.clone();
    }
}

// A simple tagged union used to demonstrate problems with aliasing.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}

fn aliasing() {
    // 2. Aliasing violation
    let ptr = Ptr::new(RefOrInt::Ref(&42));

    let mutable_ref_one = ptr.get();
    let mutable_ref_two = ptr.get();

    println!("Pointer points to: {:?}", mutable_ref_one);
    if let RefOrInt::Ref(ref addr) = mutable_ref_one {
        *mutable_ref_two = RefOrInt::Int(0xdeadbeef);

        println!("Pointer now points to: {:p}", *addr);
        println!("Dereferencing addr will now segfault: {}", **addr);
    }
}

fn main() {
    //wild_sync();
    //aliasing();
}

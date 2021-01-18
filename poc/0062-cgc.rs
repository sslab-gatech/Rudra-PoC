/*!
```rudra-poc
[target]
crate = "cgc"
version = "0.4.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/playXE/cgc/issues/5"
issue_date = 2020-12-10
unique_bugs = 3
additional_send_sync_violations = 1
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

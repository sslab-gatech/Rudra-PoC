/*!
```rudra-poc
[target]
crate = "stderr"
version = "0.8.0"

[report]
issue_url = "https://github.com/biluohc/stderr/issues/5"
issue_date = 2020-12-22
rustsec_url = "https://github.com/RustSec/advisory-db/pull/585"
rustsec_id = "RUSTSEC-2020-0109"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
rudra_report_locations = ["src/static_mut.rs:62:1: 62:40"]

[[bugs]]
analyzer = "Manual"
guide = "SendSyncVariance"
bug_class = "Other"
rudra_report_locations = []
```
!*/
#![forbid(unsafe_code)]

use stderr::StaticMut;

// A simple tagged union used to demonstrate problems with aliasing.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u128),
}

fn main() {
    let ptr = StaticMut::new(RefOrInt::Ref(&42));

    let mutable_ref_one = ptr.as_mut();
    let mutable_ref_two = ptr.as_mut();

    println!("Pointer points to: {:?}", mutable_ref_one);
    if let RefOrInt::Ref(ref addr) = mutable_ref_one {
        *mutable_ref_two = RefOrInt::Int(0xdeadbeef);

        println!("Pointer now points to: {:p}", *addr);
        println!("Dereferencing addr will now segfault: {}", **addr);
    }
}

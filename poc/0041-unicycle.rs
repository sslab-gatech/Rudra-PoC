/*!
```rudra-poc
[target]
crate = "unicycle"
version = "0.6.3"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[report]
issue_url = "https://github.com/udoprog/unicycle/issues/8"
issue_date = 2020-11-15
rustsec_url = "https://github.com/RustSec/advisory-db/pull/655"
rustsec_id = "RUSTSEC-2020-0116"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 4
rudra_report_locations = [
    "src/pin_slab.rs:43:1: 43:38",
    "src/pin_slab.rs:44:1: 44:38",
    "src/lib.rs:380:1: 380:64",
    "src/lib.rs:381:1: 381:64",
]
sendsync_details = ["NeedSend","NeedSync","NeedSend","NeedSync",]
```
!*/
#![forbid(unsafe_code)]

use unicycle::pin_slab::PinSlab;

use crossbeam_utils::thread;
use std::cell::Cell;

// A simple tagged union used to demonstrate problems with data races in Cell.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));

    let mut slab = PinSlab::new();
    slab.insert(&cell);

    thread::scope(|s| {
        s.spawn(move |_| {
            loop {
                let smuggled_cell = slab.get(0).unwrap();
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&SOME_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            if let RefOrInt::Ref(addr) = cell.get() {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u64 == &SOME_INT as *const u64 {
                    continue;
                }

                // Due to the data race, obtaining Ref(0xdeadbeef) is possible
                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

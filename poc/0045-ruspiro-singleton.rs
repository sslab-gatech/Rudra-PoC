/*!
```rudra-poc
[target]
crate = "ruspiro-singleton"
version = "0.4.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/RusPiRo/ruspiro-singleton/issues/10"
issue_date = 2020-11-16
unique_bugs = 1
additional_send_sync_violations = 1
```
!*/
#![forbid(unsafe_code)]

use ruspiro_singleton::Singleton;

use std::{cell::Cell, thread};

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

static STATIC_CELL: Singleton<Cell<RefOrInt>> =
    Singleton::lazy(&|| Cell::new(RefOrInt::Ref(&SOME_INT)));

fn main() {
    thread::spawn(move || {
        loop {
            STATIC_CELL.with_ref(|cell| {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                cell.set(RefOrInt::Ref(&SOME_INT));
                cell.set(RefOrInt::Int(0xdeadbeef));
            });
        }
    });

    STATIC_CELL.with_ref(|cell| {
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

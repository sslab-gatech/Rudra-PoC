/*!
```rudra-poc
[target]
crate = "v9"
version = "0.1.41"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/purpleposeidon/v9/issues/1"
issue_date = 2020-12-18
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

use std::cell::Cell;
use std::fmt::Debug;
use std::thread;

use v9::util::SyncRef;

static STATIC_INT: u64 = 123;

// A simple tagged union for demonstrating the data race
#[derive(Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u128),
}

#[derive(Clone)]
struct RefOrIntCell(Cell<RefOrInt>);

impl RefOrIntCell {
    pub fn new() -> Self {
        RefOrIntCell(Cell::new(RefOrInt::Ref(&STATIC_INT)))
    }
}

impl Debug for RefOrIntCell {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {
            if let RefOrInt::Ref(addr) = self.0.get() {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u64 == &STATIC_INT as *const u64 {
                    continue;
                }

                // We got an invalid reference with safe Rust code
                println!("Reference is pointing: {:p}", addr);
                println!("Dereferencing addr will segfault: {}", *addr);
            }
        }
    }
}

fn main() {
    // Creating &'static RefOrIntCell
    let ref_or_int = &*Box::leak(Box::new(RefOrIntCell::new()));
    // Creating &'static SyncRef<&'static RefOrIntCell>
    let sync_ref = &*Box::leak(Box::new(SyncRef::new(ref_or_int)));

    thread::spawn(move || {
        // Cell<_> is non-Sync, but `SyncRef` allows this type to be accessed concurrently by multiple threads
        format!("{:?}", sync_ref);
    });

    loop {
        // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
        ref_or_int.0.set(RefOrInt::Ref(&STATIC_INT));
        ref_or_int.0.set(RefOrInt::Int(0xdeadbeef));
    }
}

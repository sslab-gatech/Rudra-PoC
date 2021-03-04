/*!
```rudra-poc
[target]
crate = "internment"
version = "0.3.13"

[report]
issue_url = "https://github.com/droundy/internment/issues/20"
issue_date = 2021-03-03
rustsec_url = "https://github.com/RustSec/advisory-db/pull/807"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
rudra_report_locations = ["src/lib.rs:116:1: 116:37"]
```
!*/
#![forbid(unsafe_code)]
use internment::Intern;

use std::borrow::Borrow;
use std::cell::Cell;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

// A simple tagged union used to demonstrate problems with data races in Cell.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Foo(Cell<RefOrInt>);

impl Hash for Foo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get().hash(state);
    }
}

impl Foo {
    fn set(&self, v: RefOrInt) {
        self.0.set(v);
    }
    fn get(&self) -> RefOrInt {
        self.0.get()
    }
}

fn main() {
    let non_sync = Foo(Cell::new(RefOrInt::Ref(&SOME_INT)));
    let i0 = Arc::new(Intern::new(non_sync));

    let i1 = i0.clone();
    std::thread::spawn(move || {
        let i1 = i1;

        loop {
            // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
            i1.set(RefOrInt::Ref(&SOME_INT));
            i1.set(RefOrInt::Int(0xdeadbeef));
        }
    });
    
    loop {
        if let RefOrInt::Ref(addr) = i0.get() {
            // Hope that between the time we pattern match the object as a
            // `Ref`, it gets written to by the other thread.
            if addr as *const u64 == &SOME_INT as *const u64 {
                continue;
            }

            println!("Pointer is now: {:p}", addr);
            println!("Dereferencing addr will now segfault: {}", *addr);
        }
    }
}

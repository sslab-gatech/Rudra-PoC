/*!
```rudra-poc
[target]
crate = "parallel-event-emitter"
version = "0.2.4"

[report]
issue_url = "https://github.com/novacrazy/parallel-event-emitter/issues/2"
issue_date = 2021-03-02

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
rudra_report_locations = ["src/lib.rs:237:5: 237:50"]
```
!*/
#![forbid(unsafe_code)]
use parallel_event_emitter::ParallelEventEmitter;

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

#[derive(PartialEq, Eq, Clone)]
struct Foo(Cell<RefOrInt>);

impl Hash for Foo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get().hash(state);
    }
}

fn main() {
    let non_sync_key = Foo(Cell::new(RefOrInt::Ref(&SOME_INT)));
    let mut emit0 = ParallelEventEmitter::new();
    emit0.add_listener(
        non_sync_key,
        || Ok(()) // dummy listener
    );
    let emit0 = Arc::new(emit0);

    let emit1 = emit0.clone();
    std::thread::spawn(move || {
        let emit1 = emit1;

        emit1.event_names_visitor(|key: &Foo| {
            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                key.0.set(RefOrInt::Ref(&SOME_INT));
                key.0.set(RefOrInt::Int(0xdeadbeef));
            }
        });
    });

    emit0.event_names_visitor(|key: &Foo| {
        loop {
            if let RefOrInt::Ref(addr) = key.0.get() {
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

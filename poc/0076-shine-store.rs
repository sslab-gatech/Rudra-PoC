/*!
```rudra-poc
[target]
crate = "shine-store"
version = "0.2.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/gzp-crey/shine/issues/2"
issue_date = 2020-12-23
```
!*/
#![forbid(unsafe_code)]

use shine_store::hashstore::{HashStore, Key};

use crossbeam_utils::thread;
use std::cell::Cell;

// Key to use for HashStore
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct MyKey(i32);
impl Key for MyKey {}

// A simple tagged union used to demonstrate problems with data-races.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u128),
}

static STATIC_INT: u64 = 123;

// Type that implements From<K> for HashStore.
struct RefOrIntCell(Cell<RefOrInt>);

impl From<MyKey> for RefOrIntCell {
    fn from(key: MyKey) -> Self {
        Self(Cell::new(RefOrInt::Ref(&STATIC_INT)))
    }
}

fn main() {
    let store = HashStore::<MyKey, RefOrIntCell>::new();

    let key = MyKey(0);
    let idx = store.write().get_or_add(&key);

    thread::scope(|s| {
        s.spawn(|_| {
            let read_guard = store.read();
            let smuggled_cell = read_guard.at(&idx);

            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.0.set(RefOrInt::Ref(&STATIC_INT));
                smuggled_cell.0.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        let read_guard = store.read();
        let cell = read_guard.at(&idx);
        loop {
            if let RefOrInt::Ref(addr) = cell.0.get() {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u64 == &STATIC_INT as *const u64 {
                    continue;
                }

                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

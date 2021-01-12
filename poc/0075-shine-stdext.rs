/*!
```rudra-poc
[target]
crate = "shine-stdext"
version = "0.2.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["Manual", "SendSyncVariance"]
bug_classes = ["SendSyncVariance", "Other"]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/gzp-crey/shine/issues/1"
issue_date = 2020-12-23
```
!*/
#![forbid(unsafe_code)]

use shine_stdext::unnamedstore::Store;

use crossbeam_utils::thread;
use std::cell::Cell;

// A simple tagged union used to demonstrate problems with data-races.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u128),
}

static STATIC_INT: u64 = 123;

// 1. Store<D> implements Send/Sync for all D.
fn wild_send_sync_unnamed_store() {
    let cell = Cell::new(RefOrInt::Ref(&STATIC_INT));
    let store = Store::<&Cell<RefOrInt>>::new();

    let idx = store.try_write().unwrap().add(&cell);

    thread::scope(|s| {
        s.spawn(|_| {
            let read_guard = store.try_read().unwrap();
            let smuggled_cell = read_guard.at(&idx);

            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&STATIC_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            if let RefOrInt::Ref(addr) = cell.get() {
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

use shine_stdext::spscstate::state_channel;

// Needed because state_channel requires the type to implement Default.
#[derive(Debug, Clone)]
struct RefOrIntCellContainer(Cell<RefOrInt>);

impl Default for RefOrIntCellContainer {
    fn default() -> Self {
        Self(Cell::new(RefOrInt::Ref(&STATIC_INT)))
    }
}

// 2. shine_stdext::spscstate::RefSendBuffer automatically implements Send and
//    Sync but allows access to non-Send/Sync types.
fn wild_send_sync_state_channel() {
    let cell_container = RefOrIntCellContainer(Cell::new(RefOrInt::Ref(&STATIC_INT)));
    let (sender, receiver) = state_channel::<RefOrIntCellContainer>();

    let send_buffer = sender.send_buffer().unwrap();

    thread::scope(|s| {
        s.spawn(|_| {
            let smuggled_cell = &send_buffer.0;
            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&STATIC_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            if let RefOrInt::Ref(addr) = send_buffer.0.get() {
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

// 3. Index does not track which Store it came from allowing for aliasing
//    violations when used across multiple stores.
fn index_aliasing_violation() {
    let store = Store::<RefOrInt>::new();
    let another_store = Store::<RefOrInt>::new();

    let idx = store.try_write().unwrap().add(RefOrInt::Ref(&42));

    let mut write_guard_1 = store.try_write().unwrap();
    let mut write_guard_2 = another_store.try_write().unwrap();

    let mutable_ref_one = write_guard_1.at_mut(&idx);
    let mutable_ref_two = write_guard_2.at_mut(&idx);

    println!("Pointer points to: {:?}", mutable_ref_one);
    if let RefOrInt::Ref(ref addr) = mutable_ref_one {
        *mutable_ref_two = RefOrInt::Int(0xdeadbeef);

        println!("Pointer now points to: {:p}", *addr);
        println!("Dereferencing addr will now segfault: {}", **addr);
    }
}

fn main() {
    //wild_send_sync_unnamed_store();
    //wild_send_sync_state_channel();
    index_aliasing_violation();
}

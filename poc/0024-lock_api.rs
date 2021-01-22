/*!
```rudra-poc
[target]
crate = "lock_api"
version = "0.4.1"

[[target.peer]]
crate = "parking_lot"
version = "0.11.0"
features = ["send_guard"]

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[report]
issue_url = "https://github.com/Amanieu/parking_lot/issues/258"
issue_date = 2020-11-08
rustsec_url = "https://github.com/RustSec/advisory-db/pull/483"
rustsec_id = "RUSTSEC-2020-0070"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 5
```
!*/
#![forbid(unsafe_code)]

use parking_lot::{Mutex, MutexGuard};
use std::cell::Cell;

use crossbeam_utils::thread;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let mutex = Mutex::new(&cell);

    thread::scope(|s| {
        let guard = mutex.lock();
        // MappedMutexGuard that just returns the whole object as a "component".
        let mapped_guard = MutexGuard::map(guard, |x| x);

        let child = s.spawn(move |_| {
            let smuggled_cell = *mapped_guard;

            println!("Thread - {:p} - {:?}", smuggled_cell, smuggled_cell);
            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&SOME_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        println!("Main - {:p} - {:?}", &cell, cell);
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

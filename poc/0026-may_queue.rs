/*!
```rudra-poc
[target]
crate = "may_queue"
version = "0.1.7"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[report]
issue_url = "https://github.com/Xudong-Huang/may/issues/88"
issue_date = 2020-11-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/583"
rustsec_id = "RUSTSEC-2020-0111"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 4
rudra_report_locations = [
    "src/mpsc_list.rs:30:1: 30:36",
    "src/mpsc_list.rs:31:1: 31:36",
    "src/mpsc_list_v1.rs:147:1: 147:36",
    "src/mpsc_list_v1.rs:148:1: 148:36",
]
```
!*/
#![forbid(unsafe_code)]

use may_queue::mpsc_list::Queue;

use crossbeam_utils::thread;
use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let queue = Queue::new();
    queue.push(&cell);

    thread::scope(|s| {
        s.spawn(move |_| {
            let smuggled_cell = queue.pop().unwrap();

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

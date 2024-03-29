/*!
```rudra-poc
[target]
crate = "signal-simple"
version = "0.1.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[report]
issue_url = "https://github.com/kitsuneninetails/signal-rust/issues/2"
issue_date = 2020-11-15
rustsec_url = "https://github.com/RustSec/advisory-db/pull/694"
rustsec_id = "RUSTSEC-2020-0126"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/channel.rs:58:1: 58:42", "src/channel.rs:59:1: 59:42"]
```
!*/
#![forbid(unsafe_code)]

use signal_simple::channel::SyncChannel;

use crossbeam_utils::thread;
use std::cell::Cell;

// A simple tagged union used to demonstrate problems with data races in Cell.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));

    let channel = SyncChannel::new();
    channel.send(&cell);

    thread::scope(|s| {
        s.spawn(|_| {
            let smuggled_cell = channel.recv().unwrap();
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

                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

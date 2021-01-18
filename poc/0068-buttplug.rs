/*!
```rudra-poc
[target]
crate = "buttplug"
version = "0.10.0"

[[target.peer]]
crate = "futures"
version = "0.3.8"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/buttplugio/buttplug-rs/issues/225"
issue_date = 2020-12-18
unique_bugs = 1
additional_send_sync_violations = 1
```
!*/
#![forbid(unsafe_code)]
use buttplug::util::future::ButtplugFuture;
use futures::executor;

use std::cell::Cell;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static X: u64 = 0;

fn main() {
    let future = ButtplugFuture::default();
    let shared = future.get_state_clone();

    thread::spawn(move || {
        let shared = shared;

        let cell = Arc::new(Cell::new(RefOrInt::Int(0xdeadbeef)));
        shared.set_reply(Arc::clone(&cell));

        loop {
            cell.set(RefOrInt::Int(0xdeadbeef));
            cell.set(RefOrInt::Ref(&X))
        }
    });

    let smuggled_cell: Arc<Cell<RefOrInt>> = executor::block_on(future);
    println!("Future resolved");

    loop {
        if let RefOrInt::Ref(addr) = smuggled_cell.get() {
            if addr as *const _ as usize != 0xdeadbeef {
                continue;
            }
            // Due to the data race, obtaining Ref(0xdeadbeef) is possible
            println!("Pointer is now: {:p}", addr);
            println!("Dereferencing addr will now segfault: {}", *addr);
        }
    }
}

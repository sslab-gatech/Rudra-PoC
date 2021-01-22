/*!
```rudra-poc
[target]
crate = "bottle"
version = "1.1.0-alpha"

[[target.peer]]
crate = "async-std"
version = "1.7.0"
features = ["attributes"]

[test]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/timothee-haudebourg/bottle/issues/1"
issue_date = 2020-12-07

[[bugs]]
analyzer = "Manual"
guide = "SendSyncVariance"
bug_class = "Other"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]
#![feature(arbitrary_self_types)]

use std::time::Duration;

use async_std::future;
use bottle::{Event, EventQueue, Handler, Output, Receiver, Remote};

struct EmptyEvent;

impl Event for EmptyEvent {
    type Response = ();
}

struct UninitChecker {
    // magic value, should be always 0x12345678
    magic: u64,
}

impl UninitChecker {
    pub fn new() -> Self {
        UninitChecker {
            magic: 0x12345678
        }
    }

    pub fn validate(&self) {
        if self.magic != 0x12345678 {
            panic!("Uninitialized value access! 0x{:x}", self.magic);
        }
    }
}

impl Handler<EmptyEvent> for UninitChecker {
    fn handle(self: Receiver<Self>, _event: EmptyEvent) -> Output<'static, ()> {
        self.validate();
        Output::Now(())
    }
}

#[async_std::main]
async fn main() {
    let queue1 = EventQueue::new();
    let queue1_ref = queue1.reference();
    let queue2 = EventQueue::new();
    let queue2_ref = queue2.reference();

    let remote = Remote::from(queue1_ref, || UninitChecker::new());

    std::thread::spawn(move || {
        async_std::task::block_on(queue2.process())
    });

    if let Err(_timeout) = future::timeout(Duration::from_secs(1), queue2_ref.push(remote.clone(), EmptyEvent)).await {
        println!("Future timeout");
    }
}

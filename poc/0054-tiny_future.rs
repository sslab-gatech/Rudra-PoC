/*!
```rudra-poc
[target]
crate = "tiny_future"
version = "0.3.2"

[test]
analyzers = ["SendSyncChecker"]

[report]
issue_url = "https://github.com/KizzyCode/tiny_future/issues/1"
issue_date = 2020-12-08
```
!*/
#![forbid(unsafe_code)]

use tiny_future::Future;
use std::{thread, rc::Rc};

fn main() {
    let rc = Rc::new(());
    let rc_clone = rc.clone();

    let f = Future::with_state(());
    f.set(rc_clone);

    thread::spawn(move || {
        let smuggled_rc = f.get().unwrap();

        println!("Thread: {:p}", smuggled_rc);
        // Race the refcount with the main thread.
        for _ in 0..100_000_000 {
            smuggled_rc.clone();
        }
    });

    println!("Main:   {:p}", rc);
    for _ in 0..100_000_000 {
        rc.clone();
    }
}

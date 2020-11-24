/*!
```crux-poc
[target]
crate = "conqueue"
version = "0.3.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "QueueSender's Send trait and Sync trait should have bounds"
description = """
Hi there, we (Rust group @sslab-gatech) are scanning crates on crates.io for potential soundness bugs.
We noticed that the `QueueSender` object implements the Send and Sync traits for all types:

https://github.com/longshorej/conqueue/blob/61f02f82370eadf4bfbf9c42c7de059d622799ea/src/lib.rs#L79-L81

However, this should probably be bounded by T: Send and T: Sync.
Otherwise, it's possible to smuggle non-Send types across thread boundaries or share non-Sync types across thread boundaries.

Here's an example of a data race in safe Rust code through a `conqueue::Queue`.
"""
code_snippets = ["https://github.com/longshorej/conqueue/blob/61f02f82370eadf4bfbf9c42c7de059d622799ea/src/lib.rs#L79-L81"]
patched = []
informational = "unsound"
issue_url = "https://github.com/longshorej/conqueue/issues/9"
issue_date = 2020-11-24
```
!*/
#![forbid(unsafe_code)]

use std::ops::Deref;
use std::rc::Rc;

const NUM_CLONES: usize = 1000000;

use conqueue::*;
use crossbeam_utils::thread;

fn main() {
    let (tx, mut rx) = conqueue::Queue::unbounded();

    let rc = Rc::new(true);
    tx.push(Box::new(rc.clone()));

    // We demonstrate the issue by racing the non-atomic reference counter type `Rc` between two threads.
    thread::scope(|s| {
        let child = s.spawn(|_| {
            // &Rc<bool> sent to another thread
            let smuggled = rx.pop();
            for _ in 0..NUM_CLONES {
                std::mem::forget(smuggled.clone());
            }
        });

        // if `child.join().unwrap()` is here, the program succeeds

        for _ in 0..NUM_CLONES {
            std::mem::forget(rc.clone());
        }

        child.join().unwrap();

        // We made NUM_CLONES on both threads plus initial 1 reference.
        // But in reality we'll see that the strong_count varies across every run due to data race.
        assert_eq!(Rc::strong_count(&rc), 2 * NUM_CLONES + 1);
    })
    .unwrap();
}

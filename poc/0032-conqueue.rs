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
title = ""
description = """

"""
code_snippets = ["https://github.com/longshorej/conqueue/blob/61f02f82370eadf4bfbf9c42c7de059d622799ea/src/lib.rs#L79"]
patched = []
informational = "unsound"
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

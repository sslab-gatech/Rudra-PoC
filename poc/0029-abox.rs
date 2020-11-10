/*!
```crux-poc
[target]
crate = "abox"
version = "0.4.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "issue title"
description = """
issue description"""
code_snippets = []
patched = []
informational = "unsound"
```
!*/
#![forbid(unsafe_code)]

use std::rc::Rc;

const NUM_CLONES: usize = 1000000;

use abox::*;
use crossbeam_utils::thread;

fn main() {
    let rc = Rc::new(true);
    let b = AtomicBox::new(rc.clone());

    // We demonstrate the issue by racing the non-atomic reference counter type `Rc` between two threads.
    thread::scope(|s| {
        let child = s.spawn(|_| {
            // &Rc<bool> sent to another thread
            let smuggled = &*b.get();
            for _ in 0..NUM_CLONES {
                std::mem::forget(smuggled.clone());
            }
        });

        // if `child.join().unwrap()` is here, the program succeeds

        for _ in 0..NUM_CLONES {
            std::mem::forget(rc.clone());
        }

        child.join().unwrap();

        // We made NUM_CLONES on both threads plus initial 2 reference.
        // But in reality we'll see that the strong_count varies across every run due to data race.
        assert_eq!(Rc::strong_count(&rc), 2 * NUM_CLONES + 2);
    })
    .unwrap();
}

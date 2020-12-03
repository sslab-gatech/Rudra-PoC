/*!
```rudra-poc
[target]
crate = "cache"
version = "0.2.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "Cache's Send trait and Sync trait should have bounds"
description = """
Hi there, we (Rust group @sslab-gatech) are scanning crates on crates.io for potential soundness bugs.
We noticed that the `Cache` object implements the Send and Sync traits for all types:

https://github.com/krl/cache/blob/65e4eb4e6e40a4b4b8a9fdbd7fe4c45dd58f1637/src/lib.rs#L85-L86

However, this should probably be bounded by K: Send and K: Sync.
Otherwise, it's possible to smuggle non-Send types across thread boundaries or share non-Sync types across thread boundaries.

Here's an example of a data race in safe Rust code through a Cache.
"""
code_snippets = ["https://github.com/krl/cache/blob/65e4eb4e6e40a4b4b8a9fdbd7fe4c45dd58f1637/src/lib.rs#L85-L86"]
patched = []
informational = "unsound"
issue_url = "https://github.com/krl/cache/issues/1"
issue_date = 2020-11-24
```
!*/
#![forbid(unsafe_code)]

use std::rc::Rc;

const NUM_CLONES: usize = 1000000;

use cache::*;
use crossbeam_utils::thread;

fn main() {
    let rc = Rc::new(true);
    let cache = Cache::new(1, 4096);
    cache.insert(0, rc.clone());

    // We demonstrate the issue by racing the non-atomic reference counter type `Rc` between two threads.
    thread::scope(|s| {
        let child = s.spawn(|_| {
            // &Rc<bool> sent to another thread
            let smuggled = cache.get::<Rc<bool>>(&0).unwrap();
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

/*!
```crux-poc
[target]
crate = "try-mutex"
version = "0.2.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "TryMutex<T> lacks a Send bound on its contained type"
description = """
issue description"""
code_snippets = ["https://github.com/mpdn/try-mutex/blob/9775aef4516135958fa428ce08b346325c0493f3/src/lib.rs#L127"]
patched = []
informational = "unsound"
issue_url = "https://github.com/mpdn/try-mutex/issues/2"
issue_date = 2020-11-17
```
!*/
#![forbid(unsafe_code)]

use try_mutex::TryMutex;

use std::rc::Rc;
use crossbeam_utils::thread;

fn main() {
    let rc = Rc::new(());
    let rc_clone = rc.clone();

    let try_mutex = TryMutex::new(rc_clone);
    thread::scope(|s| {    
        s.spawn(|_| {
            let smuggled_rc = try_mutex.try_lock().unwrap();
            println!("RC in thread: {:p}", *smuggled_rc);
        });
    });
    println!("RC in main:   {:p}", rc);
}

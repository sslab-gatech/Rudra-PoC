/*!
```rudra-poc
[target]
crate = "noise_search"
version = "0.7.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
issue_date = 2020-12-10
issue_url = "https://github.com/pipedown/noise/issues/72"
```
!*/
#![forbid(unsafe_code)]

use noise_search::index::MvccRwLock;

use std::rc::Rc;

fn main() {
    let rc = Rc::new(42);

    let lock = MvccRwLock::new(rc.clone());
    std::thread::spawn(move || {
        let smuggled_rc = lock.read();

        println!("Thread: {:p}", *smuggled_rc);
        for _ in 0..100_000_000 {
            (*smuggled_rc).clone();
        }
    });

    println!("Main:   {:p}", rc);
    for _ in 0..100_000_000 {
        rc.clone();
    }
}
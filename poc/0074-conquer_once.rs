/*!
```rudra-poc
[target]
crate = "conquer-once"
version = "0.2.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.1"

[test]
analyzers = ["SendSyncVariance"]

[report]
issue_date = 2020-12-22
issue_url = "https://github.com/oliver-giersch/conquer-once/issues/3"
```
!*/
#![forbid(unsafe_code)]
use conquer_once::OnceCell;
use crossbeam_utils::thread;

use std::sync::Mutex;

fn main() {
    let once_cell = OnceCell::uninit();
    thread::scope(|s| {
        s.spawn(|_| {
            once_cell
                .try_init_once(move || {
                    let mutex_static = Box::leak(Box::new(Mutex::new(0_i32)));

                    // `MutexGuard`is `Sync`, but not `Send`.
                    let mutex_guard = mutex_static.lock().unwrap();
                    let tid = std::thread::current().id();
                    (mutex_guard, tid)
                })
                .unwrap();
        });
    })
    .unwrap();

    if let Some((smuggled_mutexguard, tid)) = once_cell.into_inner() {
        // `smuggled_mutexguard` is dropped at the end of its lexical scope.
        // This will make the parent thread unlock the Mutex which it did not lock.
        //
        // If a thread attempts to unlock a Mutex that it has not locked, undefined behavior can happen.
        // (https://github.com/rust-lang/rust/issues/23465#issuecomment-82730326)
        assert_eq!(tid, std::thread::current().id());
    }
}

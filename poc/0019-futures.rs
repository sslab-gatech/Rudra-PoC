/*!
```rudra-poc
[target]
crate = "futures"
version = "0.3.6"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
issue_url = "https://github.com/rust-lang/futures-rs/issues/2239"
issue_date = 2020-10-23
rustsec_url = "https://github.com/RustSec/advisory-db/pull/455"
rustsec_id = "RUSTSEC-2020-0059"
```
!*/
#![forbid(unsafe_code)]

use crossbeam_utils::thread;
use futures::lock::{Mutex, MutexGuard};
use std::ops::Deref;
use std::rc::Rc;

const NUM_CLONES: usize = 1000000;

fn main() {
    let mutex = Mutex::new(true);
    let mutex_guard = futures::executor::block_on(mutex.lock());

    // T: bool, U: Rc<bool>
    // MappedMutexGuard is Send+Sync and Deref to U, due to:
    // unsafe impl<T: ?Sized + Send, U: ?Sized> Send for MappedMutexGuard<'_, T, U> {}
    // unsafe impl<T: ?Sized + Sync, U: ?Sized> Sync for MappedMutexGuard<'_, T, U> {}
    let mapped_mutex_guard = MutexGuard::map(mutex_guard, |_| Box::leak(Box::new(Rc::new(true))));

    // We demonstrate the issue by racing the non-atomic reference counter type `Rc` between two threads.
    thread::scope(|s| {
        let child = s.spawn(|_| {
            // &Rc<bool> sent to another thread
            let rc_ref = mapped_mutex_guard.deref();
            for _ in 0..NUM_CLONES {
                std::mem::forget(rc_ref.clone());
            }
        });

        // if `child.join().unwrap()` is here, the program succeeds

        let rc_ref = mapped_mutex_guard.deref();
        for _ in 0..NUM_CLONES {
            std::mem::forget(rc_ref.clone());
        }

        child.join().unwrap();

        // We made NUM_CLONES on both threads plus initial 1 reference.
        // But in reality we'll see that the strong_count varies across every run due to data race.
        assert_eq!(Rc::strong_count(rc_ref), 2 * NUM_CLONES + 1);
    })
    .unwrap();
}

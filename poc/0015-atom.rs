/*!
```rudra-poc
[target]
crate = "atom"
version = "0.3.5"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/slide-rs/atom/issues/13"
issue_date = 2020-09-21
rustsec_url = "https://github.com/RustSec/advisory-db/pull/390"
rustsec_id = "RUSTSEC-2020-0044"
```
!*/
#![forbid(unsafe_code)]

use atom::Atom;
use std::rc::Rc;
use std::sync::mpsc;
use std::{mem, thread, time};

const NUM_CLONES: usize = 10000000;

fn main() {
    // The following code demonstrates how the Atom type unsoundly allows any
    // type to be sent across threads using the Rc type.
    //
    // The Rc type in Rust does not implement Send since it is supposed to be a
    // low overhead smart pointer class. Its reference counting mechanism is
    // non-atomic meaning that it is unsafe to use across two threads as they
    // can potentially race when updating the count. This can potentially lead
    // to memory safety issues like use-after-frees.
    //
    // As such, we demonstrate this issue by creating a Rc on one thread and
    // creating `n` clones of it. Simultaneously, we send the Rc to another
    // thread and have it do the same. With an atomic counting mechanism we
    // would expect the reference count to be increased by `2n`. However, since
    // Rc is not atomic and was never meant to be sent across threads, we see
    // wildly varying values on every run.
    let x = Rc::new(());
    let y = Rc::clone(&x);

    // Place our Rc in the Atom through a Box.
    let shared_atom = Atom::empty();
    shared_atom.swap(Box::new(x));

    let child = thread::spawn(move || {
        // We now have the Rc shared across a thread.
        let x = shared_atom.take().unwrap();
        for _ in 0..NUM_CLONES {
            let clone = x.clone();
            mem::forget(clone);
        }
    });

    for _ in 0..NUM_CLONES {
        let clone = y.clone();
        mem::forget(clone);
    }

    // Wait for the spawned thread to finish its cloning.
    child.join().unwrap();

    // We made NUM_CLONES on both threads plus 2 references in the main thread.
    // But in reality we'll see that the strong_count varies across every run.
    assert_eq!(Rc::strong_count(&y), (NUM_CLONES * 2) + 2);
}

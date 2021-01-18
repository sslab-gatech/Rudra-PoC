/*!
```rudra-poc
[target]
crate = "kekbit"
version = "0.3.3"

[[target.peer]]
crate = "tempdir"
version = "0.3.7"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/motoras/kekbit/issues/34"
issue_date = 2020-12-18
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

use std::marker::PhantomData;
use std::thread;

use kekbit::api::Handler;
use kekbit::core::{shm_writer, Metadata, TickUnit::Nanos};

// non-Send type that panics when dropped in a wrong thread
struct NonSend {
    created_thread: thread::ThreadId,
    // Ensure `NonSend` type does not implement `Send` trait
    _marker: PhantomData<*mut ()>,
}

impl NonSend {
    pub fn new() -> Self {
        NonSend {
            created_thread: thread::current().id(),
            _marker: PhantomData,
        }
    }
}

impl Drop for NonSend {
    fn drop(&mut self) {
        if thread::current().id() != self.created_thread {
            panic!("NonSend destructor is running on a wrong thread!");
        }
    }
}

impl Handler for NonSend {}

fn main() {
    // Example code from: https://docs.rs/kekbit/0.3.3/kekbit/core/fn.shm_writer.html#examples
    const FOREVER: u64 = 99_999_999_999;
    let writer_id = 1850;
    let channel_id = 42;
    let capacity = 3000;
    let max_msg_len = 100;
    let metadata = Metadata::new(writer_id, channel_id, capacity, max_msg_len, FOREVER, Nanos);
    let test_tmp_dir = tempdir::TempDir::new("kekbit").unwrap();

    let writer = shm_writer(&test_tmp_dir.path(), &metadata, NonSend::new()).unwrap();

    let handle = thread::spawn(move || {
        // `NonSend` is sent to another thread via `ShmWriter`
        drop(writer);
    });

    handle.join().unwrap();
}

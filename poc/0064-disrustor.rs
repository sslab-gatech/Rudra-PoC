/*!
```rudra-poc
[target]
crate = "disrustor"
version = "0.2.0"

[[target.peer]]
crate = "static_assertions"
version = "1.1.0"

[test]
analyzers = []

[report]
issue_url = "https://github.com/sklose/disrustor/issues/1"
issue_date = 2020-12-17
```
!*/
#![forbid(unsafe_code)]

#[macro_use]
extern crate static_assertions;

use disrustor::internal::RingBuffer;

use std::marker::PhantomData;
use std::thread;

struct NonSend {
    created_thread: thread::ThreadId,
    // Mark this struct as `NonSend`
    _marker: PhantomData<*mut ()>,
}

assert_not_impl_all!(NonSend: Send);

impl Default for NonSend {
    fn default() -> Self {
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

fn main() {
    let buffer = RingBuffer::<NonSend>::new(1);

    let handle = thread::spawn(move || {
        drop(buffer);
    });

    handle.join().unwrap();
}

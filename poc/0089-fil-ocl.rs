/*!
```rudra-poc
[target]
crate = "fil-ocl"
version = "0.19.4"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["PanicSafety"]

[report]
rustsec_url = "https://github.com/RustSec/advisory-db/pull/587"
issue_url = "https://github.com/cogciprocate/ocl/issues/194"
issue_date = 2021-01-04
unique_bugs = 2
```
!*/
#![forbid(unsafe_code)]

// `fil_ocl` crate requires OpenCL to be installed in order to build & run.
// CI will probably fail to build & run this poc.
// Thus I changed the file extension of this PoC so that our CI will not build this example.

use fil_ocl::{Event, EventList};
use std::convert::Into;

struct Foo(Option<i32>);

impl Into<Event> for Foo {
    fn into(self) -> Event {
        /*
        According to the docs, `Into<T>` implementations shouldn't panic.
        However rustc doesn't check whether panics can happen in the Into implementation,
        so it's possible for a user-provided `into()` to panic..
        */
        println!("LOUSY PANIC : {}", self.0.unwrap());

        Event::empty()
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("I'm dropping");
    }
}

fn main() {
    let eventlist: EventList = [Foo(None)].into();
    dbg!(eventlist);
}

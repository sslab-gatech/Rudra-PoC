/*!
```rudra-poc
[target]
crate = "ocl"
version = "0.19.3"

[report]
issue_url = "https://github.com/cogciprocate/ocl/issues/198"
issue_date = 2020-02-03

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
```
!*/
#![forbid(unsafe_code)]

use ocl::{Event, EventList};

struct MyIntoEventType(u32);

impl Drop for MyIntoEventType {
    fn drop(&mut self) {
        println!("Dropping the MyIntoEventType");
    }
}

impl Into<Event> for MyIntoEventType {
    fn into(self) -> Event {
        panic!("Panicking in Into");
    }
}

fn main() {
    let slice = [MyIntoEventType(1)];
    let event_list = EventList::from(slice);
}

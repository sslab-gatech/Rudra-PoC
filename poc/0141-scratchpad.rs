/*!
```rudra-poc
[target]
crate = "scratchpad"
version = "1.3.0"

[report]
issue_url = "https://github.com/okready/scratchpad/issues/1"
issue_date = 2021-02-18
rustsec_url = "https://github.com/RustSec/advisory-db/pull/793"
rustsec_id = "RUSTSEC-2021-0030"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 2
rudra_report_locations = ["src/traits.rs:867:5: 878:6", "src/traits.rs:901:5: 914:6"]
```
!*/
#![forbid(unsafe_code)]
use scratchpad::SliceLike;
use scratchpad::SliceMoveSource;

#[derive(Clone, Debug)]
struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        // This message is printed twice, indicating double-free.
        println!("I'm dropping {}", self.0);
    }
}

fn main() {
    let mut mybox = [Foo(1234); 1];

    mybox.move_elements(|s| {
        panic!("Unexpected panic");
        dbg!(s);
    });
}

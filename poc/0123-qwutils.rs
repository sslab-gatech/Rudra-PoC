/*!
```rudra-poc
[target]
crate = "qwutils"
version = "0.2.0"

[report]
issue_url = "https://github.com/qwertz19281/rust_utils/issues/3"
issue_date = 2021-02-03
rustsec_url = "https://github.com/RustSec/advisory-db/pull/747"
rustsec_id = "RUSTSEC-2021-0018"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/imp/vec.rs:66:15: 88:6"]
```
!*/
#![forbid(unsafe_code)]

use qwutils::*;

// Type with a Clone() implementation that panics.
struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

impl Clone for DropDetector {
    fn clone(&self) -> Self {
        panic!("DropDetector {} panic on clone()", self.0);
    }
}


fn main() {
    let mut a = vec![DropDetector(1), DropDetector(2)];
    let b = [DropDetector(3)];

    a.insert_slice_clone(0, &b);
}
/*!
```rudra-poc
[target]
crate = "simple-slab"
version = "0.3.2"

[test]
analyzers = ["UnsafeDestructor"]
bug_classes = ["Other"]

[report]
issue_url = "https://github.com/nathansizemore/simple-slab/issues/2"
issue_date = 2020-09-03
rustsec_url = "https://github.com/RustSec/advisory-db/pull/376"
rustsec_id = "RUSTSEC-2020-0039"
```
!*/
#![forbid(unsafe_code)]

mod boilerplate;

use simple_slab::Slab;

#[derive(Debug, PartialEq)]
struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    boilerplate::init();

    let mut slab = Slab::with_capacity(2);
    slab.insert(DropDetector(123));
    slab.insert(DropDetector(456));

    // 1. No boundary checking leads to OOB read in `index()`
    println!("{:?}", slab[20]);

    // 2. Memory leak / uninitialized memory access in `remove()`
    // element should be copied from `len - 1`, not `len`
    assert_eq!(slab.remove(0).0, 123);
    assert_eq!(slab[0].0, 456); // copied from uninitialized region `slab[2]`
}

/*!
```rudra-poc
[target]
crate = "containers"
version = "0.9.10"

[[target.peer]]
crate = "default_allocator"
version = "0.3.0"

[[target.peer]]
crate = "rel"
version = "0.2.0"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["PanicSafety"]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/strake/containers.rs/issues/2"
issue_date = 2021-01-12
rustsec_url = "https://github.com/RustSec/advisory-db/pull/606"
unique_bugs = 2
```
!*/
#![forbid(unsafe_code)]

// Tested with `rustc 1.50.0-nightly (7f9c43cf9 2020-12-23)` on Ubuntu 18.04
use containers::collections::b_tree::BTree; // containers = "0.9.10"
use default_allocator::Heap; // default_allocator = "0.3"
use rel::Core; // rel = "0.2"

fn main() {
    if let Some(mut btree) = BTree::<i32, Box<u64>, Core, Heap>::new(Core, 20) {
        if btree.insert(2, Box::new(1)).is_ok() {
            while btree
                .insert_with(2, |x| {
                    let ret = match x {
                        Some(str) => str,
                        None => Box::new(0),
                    };
                    None::<Option<u64>>.unwrap();
                    return ret;
                })
                .is_err()
            {}
        }
    }
}

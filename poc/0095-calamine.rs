/*!
```rudra-poc
[target]
crate = "calamine"
version = "0.16.2"

[test]
analyzers = ["Manual", "UnsafeDataflow"]
bug_classes = ["UninitExposure", "Other"]

[report]
issue_url = "https://github.com/tafia/calamine/issues/199"
issue_date = 2021-01-06
rustsec_url = "https://github.com/RustSec/advisory-db/pull/594"
unique_bugs = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

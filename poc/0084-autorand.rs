/*!
```rudra-poc
[target]
crate = "autorand"
version = "0.2.2"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["PanicSafety"]

[report]
issue_url = "https://github.com/mersinvald/autorand-rs/issues/5"
issue_date = 2020-12-31
rustsec_url = "https://github.com/RustSec/advisory-db/pull/612"
rustsec_id = "RUSTSEC-2020-0103"
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

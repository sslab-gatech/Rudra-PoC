/*!
```rudra-poc
[target]
crate = "columnar"
version = "0.0.19"

[report]
issue_url = "https://github.com/frankmcsherry/columnar/issues/6"
issue_date = 2021-01-07
rustsec_url = "https://github.com/RustSec/advisory-db/pull/662"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/lib.rs:290:5: 305:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

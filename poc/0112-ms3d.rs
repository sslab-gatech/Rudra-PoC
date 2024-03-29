/*!
```rudra-poc
[target]
crate = "ms3d"
version = "0.1.2"

[report]
issue_url = "https://github.com/andrewhickman/ms3d/issues/1"
issue_date = 2021-01-26
rustsec_url = "https://github.com/RustSec/advisory-db/pull/723"
rustsec_id = "RUSTSEC-2021-0016"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/read.rs:19:5: 26:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "marc"
version = "1.5.0"

[report]
issue_url = "https://github.com/blackbeam/rust-marc/issues/7"
issue_date = 2021-01-26
rustsec_url = "https://github.com/RustSec/advisory-db/pull/699"
rustsec_id = "RUSTSEC-2021-0014"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/lib.rs:110:5: 139:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

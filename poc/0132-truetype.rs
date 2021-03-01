/*!
```rudra-poc
[target]
crate = "truetype"
version = "0.30.0"
indexed_version = "0.29.0"

[report]
issue_url = "https://github.com/bodoni/truetype/issues/11"
issue_date = 2021-02-17
rustsec_url = "https://github.com/RustSec/advisory-db/pull/786"
rustsec_id = "RUSTSEC-2021-0029"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/tape.rs:50:5: 55:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

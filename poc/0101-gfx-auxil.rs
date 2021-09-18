/*!
```rudra-poc
[target]
crate = "gfx-auxil"
version = "0.7.0"
indexed_name = "gfx-hal"
indexed_version = "0.5.3"

[report]
issue_url = "https://github.com/gfx-rs/gfx/issues/3567"
issue_date = 2021-01-07
rustsec_url = "https://github.com/RustSec/advisory-db/pull/681"
rustsec_id = "RUSTSEC-2021-0091"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/pso/mod.rs:275:1: 311:2"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

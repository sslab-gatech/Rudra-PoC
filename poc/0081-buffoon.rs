/*!
```rudra-poc
[target]
crate = "buffoon"
version = "0.5.0"

[report]
issue_url = "https://github.com/carllerche/buffoon/issues/2"
issue_date = 2020-12-31
rustsec_url = "https://github.com/RustSec/advisory-db/pull/663"
rustsec_id = "RUSTSEC-2020-0154"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/input_stream.rs:98:5: 121:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

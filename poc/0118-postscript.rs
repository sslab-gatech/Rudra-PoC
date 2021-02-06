/*!
```rudra-poc
[target]
crate = "postscript"
version = "0.13.2"

[report]
issue_url = "https://github.com/bodoni/postscript/issues/1"
issue_date = 2021-01-30
rustsec_url = "https://github.com/RustSec/advisory-db/pull/728"
rustsec_id = "RUSTSEC-2021-0017"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/tape.rs:102:13: 107:14"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

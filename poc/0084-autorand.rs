/*!
```rudra-poc
[target]
crate = "autorand"
version = "0.2.2"

[report]
issue_url = "https://github.com/mersinvald/autorand-rs/issues/5"
issue_date = 2020-12-31
rustsec_url = "https://github.com/RustSec/advisory-db/pull/612"
rustsec_id = "RUSTSEC-2020-0103"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/lib.rs:161:13: 169:14"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

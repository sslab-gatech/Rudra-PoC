/*!
```rudra-poc
[target]
crate = "csv-sniffer"
version = "0.1.1"

[report]
issue_url = "https://github.com/jblondin/csv-sniffer/issues/1"
issue_date = 2021-01-05
rustsec_url = "https://github.com/RustSec/advisory-db/pull/666"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/snip.rs:7:1: 36:2"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

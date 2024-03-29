/*!
```rudra-poc
[target]
crate = "flumedb"
version = "0.1.4"
indexed_version = "0.1.3"

[report]
issue_url = "https://github.com/sunrise-choir/flumedb-rs/issues/10"
issue_date = 2021-01-07
rustsec_url = "https://github.com/RustSec/advisory-db/pull/661"
rustsec_id = "RUSTSEC-2021-0086"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
rudra_report_locations = ["src/go_offset_log.rs:212:1: 263:2", "src/offset_log.rs:330:1: 358:2"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

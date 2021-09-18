/*!
```rudra-poc
[target]
crate = "acc_reader"
version = "2.0.0"

[report]
issue_url = "https://github.com/netvl/acc_reader/issues/1"
issue_date = 2020-12-27
rustsec_url = "https://github.com/RustSec/advisory-db/pull/664"
rustsec_id = "RUSTSEC-2020-0155"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
rudra_report_locations = ["src/lib.rs:245:5: 266:6", "src/lib.rs:194:5: 219:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

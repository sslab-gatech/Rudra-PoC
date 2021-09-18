/*!
```rudra-poc
[target]
crate = "bronzedb-protocol"
version = "0.1.0"

[report]
issue_url = "https://github.com/Hexilee/BronzeDB/issues/1"
issue_date = 2021-01-03
rustsec_url = "https://github.com/RustSec/advisory-db/pull/659"
rustsec_id = "RUSTSEC-2021-0084"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
rudra_report_locations = [
    "src/ext.rs:33:5: 40:6",
    "src/ext.rs:42:5: 49:6",
]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

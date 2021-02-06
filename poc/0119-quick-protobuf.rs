/*!
```rudra-poc
[target]
crate = "quick-protobuf"
version = "0.8.0"

[report]
issue_url = "https://github.com/tafia/quick-protobuf/issues/186"
issue_date = 2021-01-30

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/reader.rs:552:5: 560:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

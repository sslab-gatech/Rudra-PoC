/*!
```rudra-poc
[target]
crate = "ruyi"
version = "0.1.6"

[report]
issue_url = "https://github.com/agemocui/ruyi/issues/1"
issue_date = 2021-02-20

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/buf/mod.rs:487:5: 522:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "ot"
version = "0.2.0"

[report]
issue_url = "https://github.com/Shawak/otbmview/issues/1"
issue_date = 2021-01-30

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/mem_read.rs:25:5: 31:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
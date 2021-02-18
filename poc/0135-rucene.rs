/*!
```rudra-poc
[target]
crate = "rucene"
version = "0.1.1"

[report]
issue_url = "https://github.com/zhihu/rucene/issues/12"
issue_date = 2021-02-17

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/core/store/io/data_input.rs:205:5: 222:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "speedy"
version = "0.7.1"

[report]
issue_url = "https://github.com/koute/speedy/issues/10"
issue_date = 2021-02-20

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/reader.rs:155:5: 187:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

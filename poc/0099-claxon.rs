/*!
```rudra-poc
[target]
crate = "claxon"
version = "0.4.3"

[report]
issue_url = "https://github.com/ruuda/claxon/issues/26"
issue_date = 2021-01-07

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
rudra_report_locations = ["src/metadata.rs:397:1: 495:2", "src/metadata.rs:506:1: 531:2"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

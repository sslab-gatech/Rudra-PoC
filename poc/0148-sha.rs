/*!
```rudra-poc
[target]
crate = "sha"
version = "1.0.3"

[report]
issue_url = "https://github.com/andydude/rust-sha/issues/2"
issue_date = 2021-02-20

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
rudra_report_locations = ["src/utils.rs:19:5: 24:6", "src/utils.rs:91:5: 120:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

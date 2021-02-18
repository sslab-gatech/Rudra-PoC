/*!
```rudra-poc
[target]
crate = "skulpin-renderer"
version = "0.3.1"

[report]
issue_url = "https://github.com/aclysma/skulpin/issues/87"
issue_date = 2021-02-17

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/util.rs:26:1: 60:2"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

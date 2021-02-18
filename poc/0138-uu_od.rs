/*!
```rudra-poc
[target]
crate = "uu_od"
version = "0.0.1"

[report]
issue_url = "https://github.com/uutils/coreutils/issues/1729"
issue_date = 2021-02-17

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/partialreader.rs:32:5: 67:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

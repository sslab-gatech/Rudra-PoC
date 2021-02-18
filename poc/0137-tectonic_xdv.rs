/*!
```rudra-poc
[target]
crate = "tectonic_xdv"
version = "0.1.11"

[report]
issue_url = "https://github.com/tectonic-typesetting/tectonic/issues/752"
issue_date = 2021-02-17

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/lib.rs:218:5: 261:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

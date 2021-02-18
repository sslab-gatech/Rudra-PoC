/*!
```rudra-poc
[target]
crate = "slice"
version = "0.0.4"

[report]
issue_url = "https://github.com/hinaria/slice/issues/2"
issue_date = 2021-02-17

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/lib.rs:186:5: 207:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

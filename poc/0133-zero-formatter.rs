/*!
```rudra-poc
[target]
crate = "zero-formatter"
version = "0.1.0"

[report]
issue_url = "https://github.com/pocketberserker/zero-formatter.rs/issues/1"
issue_date = 2021-02-17

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/primitive.rs:104:5: 114:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "libretro-backend"
version = "0.2.1"

[report]
issue_url = "https://github.com/koute/libretro-backend/issues/17"
issue_date = 2021-02-05

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/lib.rs:327:5: 381:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
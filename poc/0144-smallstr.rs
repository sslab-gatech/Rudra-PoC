/*!
```rudra-poc
[target]
crate = "smallstr"
version = "0.2.0"

[report]
issue_date = 2021-02-19
issue_url = "https://github.com/murarth/smallstr/issues/12"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/string.rs:379:5: 410:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("Reported without PoC as duplicate of https://github.com/rust-lang/rust/issues/78498")
}
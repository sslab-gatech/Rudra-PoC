/*!
```rudra-poc
[target]
crate = "telemetry"
version = "0.1.1"

[report]
issue_url = "https://github.com/Yoric/telemetry.rs/issues/45"
issue_date = 2021-02-17

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/misc.rs:138:1: 151:2"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

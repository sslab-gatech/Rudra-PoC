/*!
```rudra-poc
[target]
crate = "tensorflow"
version = "0.15.0"

[report]
issue_url = "https://github.com/tensorflow/rust/issues/284"
issue_date = 2020-12-08

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/lib.rs:994:1: 994:66", "src/lib.rs:995:1: 995:66"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

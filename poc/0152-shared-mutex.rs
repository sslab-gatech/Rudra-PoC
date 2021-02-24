/*!
```rudra-poc
[target]
crate = "shared-mutex"
version = "0.3.1"

[report]
issue_url = "https://github.com/reem/rust-shared-mutex/issues/2"
issue_date = 2021-02-24

[[bugs]]
analyzer = "Manual"
guide = "UnsafeDataflow"
bug_class = "SendSyncVariance"
rudra_report_locations = []
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("Issue reported without PoC")
}
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
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

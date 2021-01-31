/*!
```rudra-poc
[target]
crate = "quick-protobuf"
version = "0.8.0"

[report]
issue_url = "https://github.com/tafia/quick-protobuf/issues/186"
issue_date = 2021-01-30

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

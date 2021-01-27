/*!
```rudra-poc
[target]
crate = "messagepack-rs"
version = "0.8.0"

[report]
issue_url = "https://github.com/otake84/messagepack-rs/issues/2"
issue_date = 2021-01-26

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 4
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

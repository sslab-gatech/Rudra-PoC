/*!
```rudra-poc
[target]
crate = "cdr"
version = "0.2.3"

[report]
issue_url = "https://github.com/hrektts/cdr-rs/issues/10"
issue_date = 2021-01-02

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

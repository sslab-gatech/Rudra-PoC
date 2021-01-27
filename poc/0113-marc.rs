/*!
```rudra-poc
[target]
crate = "marc"
version = "1.5.0"

[report]
issue_url = "https://github.com/blackbeam/rust-marc/issues/7"
issue_date = 2021-01-26

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "balloons"
version = "0.1.0"

[report]
issue_date = 2021-02-04
issue_url = "https://github.com/yangby-cryptape/rust-balloons/issues/1"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
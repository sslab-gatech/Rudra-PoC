/*!
```rudra-poc
[target]
crate = "ms3d"
version = "0.1.2"

[report]
issue_url = "https://github.com/andrewhickman/ms3d/issues/1"
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

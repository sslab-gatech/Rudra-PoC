/*!
```rudra-poc
[target]
crate = "office"
version = "0.8.1"

[report]
issue_date = 2021-02-04
issue_url = "https://github.com/RustSec/advisory-db/issues/742"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("Duplicate of 0095-calamine. This PoC is around for tracking.")
}
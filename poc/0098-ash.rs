/*!
```rudra-poc
[target]
crate = "ash"
version = "0.31.0"

[report]
issue_url = "https://github.com/MaikKlein/ash/issues/354"
issue_date = 2021-01-07
rustsec_url = "https://github.com/RustSec/advisory-db/pull/680"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

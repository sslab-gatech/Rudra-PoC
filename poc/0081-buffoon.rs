/*!
```rudra-poc
[target]
crate = "buffoon"
version = "0.5.0"

[report]
issue_url = "https://github.com/carllerche/buffoon/issues/2"
issue_date = 2020-12-31
rustsec_url = "https://github.com/RustSec/advisory-db/pull/663"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

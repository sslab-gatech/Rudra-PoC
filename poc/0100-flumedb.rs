/*!
```rudra-poc
[target]
crate = "flumedb"
version = "0.1.4"

[report]
issue_url = "https://github.com/sunrise-choir/flumedb-rs/issues/10"
issue_date = 2021-01-07
rustsec_url = "https://github.com/RustSec/advisory-db/pull/661"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "calamine"
version = "0.16.2"

[report]
issue_url = "https://github.com/tafia/calamine/issues/199"
issue_date = 2021-01-06
rustsec_url = "https://github.com/RustSec/advisory-db/pull/594"
rustsec_id = "RUSTSEC-2021-0015"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"

[[bugs]]
analyzer = "Manual"
guide = "UnsafeDataflow"
bug_class = "Other"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

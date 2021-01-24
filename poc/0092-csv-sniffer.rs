/*!
```rudra-poc
[target]
crate = "csv-sniffer"
version = "0.1.1"

[report]
issue_url = "https://github.com/jblondin/csv-sniffer/issues/1"
issue_date = 2021-01-05
rustsec_url = "https://github.com/RustSec/advisory-db/pull/666"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

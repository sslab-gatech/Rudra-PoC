/*!
```rudra-poc
[target]
crate = "outer_cgi"
version = "0.2.0"

[report]
issue_url = "https://github.com/SolraBizna/outer_cgi/issues/1"
issue_date = 2021-01-31
rustsec_url = "https://github.com/RustSec/advisory-db/pull/864"
rustsec_id = "RUSTSEC-2021-0051"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/fcgi.rs:172:5: 208:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

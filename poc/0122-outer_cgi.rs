/*!
```rudra-poc
[target]
crate = "outer_cgi"
version = "0.2.0"

[report]
issue_url = "https://github.com/SolraBizna/outer_cgi/issues/1"
issue_date = 2021-01-31

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
rudra_report_locations = ["src/fcgi.rs:172:5: 208:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

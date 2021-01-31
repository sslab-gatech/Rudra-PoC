/*!
```rudra-poc
[target]
crate = "osm_pbf_iter"
version = "0.2.0"

[report]
issue_url = "https://github.com/astro/rust-osm-pbf-iter/issues/8"
issue_date = 2021-01-30

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

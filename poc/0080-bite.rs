/*!
```rudra-poc
[target]
crate = "bite"
version = "0.0.5"

[test]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/hinaria/bite/issues/1"
issue_date = 2020-12-31
rustsec_url = "https://github.com/RustSec/advisory-db/pull/593"
rustsec_id = "RUSTSEC-2020-0153"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/bite/read.rs:263:5: 278:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

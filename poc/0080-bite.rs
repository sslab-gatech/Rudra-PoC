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

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "array-tools"
version = "0.2.10"

[test]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/L117/array-tools/issues/2"
issue_date = 2020-12-31
rustsec_url = "https://github.com/RustSec/advisory-db/pull/665"
rustsec_id = "RUSTSEC-2020-0132"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/lib.rs:220:5: 235:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

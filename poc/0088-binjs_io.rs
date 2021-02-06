/*!
```rudra-poc
[target]
crate = "binjs_io"
version = "0.2.1"

[test]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/binast/binjs-ref/issues/460"
issue_date = 2021-01-03
rustsec_url = "https://github.com/RustSec/advisory-db/pull/660"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 4
rudra_report_locations = [
    "src/bytes/compress.rs:183:5: 262:6",
    "src/util.rs:36:5: 50:6",
    "src/multipart/read.rs:30:5: 36:6",
    "src/multipart/read.rs:42:5: 56:6",
]
```
!*/
fn main() {
    panic!("This crate fails to build on CI");
}

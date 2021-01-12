/*!
```rudra-poc
[target]
crate = "array-tools"
version = "0.2.10"

[test]
analyzers = ["UnsafeDataflow"]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/L117/array-tools/issues/2"
issue_date = 2020-12-31
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "pumpkindb_client"
version = "0.2.0"

[test]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/PumpkinDB/PumpkinDB/issues/352"
issue_date = 2021-01-31

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

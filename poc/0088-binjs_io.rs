/*!
```rudra-poc
[target]
crate = "binjs_io"
version = "0.2.1"

[test]
analyzers = ["PanicSafety"]
cargo_toolchain = "1.20.0"

[report]
issue_url = "https://github.com/binast/binjs-ref/issues/460"
issue_date = 2021-01-03
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
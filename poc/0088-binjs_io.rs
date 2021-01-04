/*!
```rudra-poc
#[target]
#crate = "binjs_io"
#version = "0.2.1"

# Dummy Target, just to make CI work..
# `binjs_io` crate currently fails to build.
[target]
crate = "rulinalg"
version = "0.4.2"

[test]
analyzers = ["PanicSafety"]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/binast/binjs-ref/issues/460"
issue_date = 2021-01-03
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
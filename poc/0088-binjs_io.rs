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

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 4
```
!*/

fn main() {
    panic!("This crate fails to build on CI");
}

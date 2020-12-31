/*!
```rudra-poc
[target]
crate = "bite"
version = "0.0.5"

[test]
analyzers = ["PanicSafety"]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/hinaria/bite/issues/1"
issue_date = 2020-12-31
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
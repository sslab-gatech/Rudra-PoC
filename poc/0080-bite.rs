/*!
```rudra-poc
[target]
crate = "bite"
version = "0.0.5"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/hinaria/bite/issues/1"
issue_date = 2020-12-31
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

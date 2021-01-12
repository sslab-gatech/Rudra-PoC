/*!
```rudra-poc
[target]
crate = "columnar"
version = "0.0.19"

[test]
analyzers = ["UnsafeDataflow"]

[report]
issue_url = "https://github.com/frankmcsherry/columnar/issues/6"
issue_date = 2021-01-07
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

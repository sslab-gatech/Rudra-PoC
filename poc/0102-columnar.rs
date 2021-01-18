/*!
```rudra-poc
[target]
crate = "columnar"
version = "0.0.19"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]

[report]
issue_url = "https://github.com/frankmcsherry/columnar/issues/6"
issue_date = 2021-01-07
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

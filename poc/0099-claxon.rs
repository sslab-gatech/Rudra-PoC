/*!
```rudra-poc
[target]
crate = "claxon"
version = "0.4.3"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]

[report]
issue_url = "https://github.com/ruuda/claxon/issues/26"
issue_date = 2021-01-07
unique_bugs = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

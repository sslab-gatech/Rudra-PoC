/*!
```rudra-poc
[target]
crate = "cdr"
version = "0.2.3"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]

[report]
issue_url = "https://github.com/hrektts/cdr-rs/issues/10"
issue_date = 2021-01-02
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

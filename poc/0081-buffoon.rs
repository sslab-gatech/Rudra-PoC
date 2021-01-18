/*!
```rudra-poc
[target]
crate = "buffoon"
version = "0.5.0"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]

[report]
issue_url = "https://github.com/carllerche/buffoon/issues/2"
issue_date = 2020-12-31
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

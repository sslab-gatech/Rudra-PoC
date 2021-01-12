/*!
```rudra-poc
[target]
crate = "ash"
version = "0.31.0"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]

[report]
issue_url = "https://github.com/MaikKlein/ash/issues/354"
issue_date = 2021-01-07
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

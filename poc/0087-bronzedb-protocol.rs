/*!
```rudra-poc
[target]
crate = "bronzedb-protocol"
version = "0.1.0"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]

[report]
issue_url = "https://github.com/Hexilee/BronzeDB/issues/1"
issue_date = 2021-01-03
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

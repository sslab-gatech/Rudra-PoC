/*!
```rudra-poc
[target]
crate = "rusb"
version = "0.6.5"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/a1ien/rusb/issues/44"
issue_date = 2020-12-18
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

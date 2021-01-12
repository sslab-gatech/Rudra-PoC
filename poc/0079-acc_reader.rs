/*!
```rudra-poc
[target]
crate = "acc_reader"
version = "2.0.0"

[test]
analyzers = ["UnsafeDataflow"]

[report]
issue_url = "https://github.com/netvl/acc_reader/issues/1"
issue_date = 2020-12-27
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

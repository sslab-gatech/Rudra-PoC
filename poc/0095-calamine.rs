/*!
```rudra-poc
[target]
crate = "calamine"
version = "0.16.2"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/tafia/calamine/issues/199"
issue_date = 2021-01-06
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
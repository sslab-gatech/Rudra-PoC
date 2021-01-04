/*!
```rudra-poc
[target]
crate = "elementwise"
version = "0.3.2"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/tspooner/elementwise/issues/1"
issue_date = 2021-01-02
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
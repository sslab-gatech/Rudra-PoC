/*!
```rudra-poc
[target]
crate = "buffoon"
version = "0.5.0"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/carllerche/buffoon/issues/2"
issue_date = 2020-12-31
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
/*!
```rudra-poc
[target]
crate = "autorand"
version = "0.2.2"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/mersinvald/autorand-rs/issues/5"
issue_date = 2020-12-31
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
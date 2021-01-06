/*!
```rudra-poc
[target]
crate = "glium"
version = "0.29.0"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/glium/glium/issues/1907"
issue_date = 2021-01-06
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
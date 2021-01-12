/*!
```rudra-poc
[target]
crate = "claxon"
version = "0.4.3"

[test]
analyzers = ["UnsafeDataflow"]

[report]
issue_url = "https://github.com/ruuda/claxon/issues/26"
issue_date = 2021-01-07
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

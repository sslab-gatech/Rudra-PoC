/*!
```rudra-poc
[target]
crate = "gfx-auxil"
version = "0.7.0"

[test]
analyzers = ["UnsafeDataflow"]

[report]
issue_url = "https://github.com/gfx-rs/gfx/issues/3567"
issue_date = 2021-01-07
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

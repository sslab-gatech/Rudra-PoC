/*!
```rudra-poc
[target]
crate = "bra"
version = "0.1.0"

[test]
analyzers = ["UnsafeDataflow"]

[report]
issue_url = "https://github.com/Enet4/bra-rs/issues/1"
issue_date = 2021-01-02
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

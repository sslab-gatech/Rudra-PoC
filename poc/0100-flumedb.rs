/*!
```rudra-poc
[target]
crate = "flumedb"
version = "0.1.4"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/sunrise-choir/flumedb-rs/blob/master/src/offset_log.rs"
issue_date = 2021-01-07
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
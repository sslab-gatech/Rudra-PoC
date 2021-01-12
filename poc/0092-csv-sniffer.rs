/*!
```rudra-poc
[target]
crate = "csv-sniffer"
version = "0.1.1"

[test]
analyzers = ["UnsafeDataflow"]

[report]
issue_url = "https://github.com/jblondin/csv-sniffer/issues/1"
issue_date = 2021-01-05
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

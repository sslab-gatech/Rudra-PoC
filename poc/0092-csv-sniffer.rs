/*!
```rudra-poc
[target]
crate = "csv-sniffer"
version = "0.1.1"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]

[report]
issue_url = "https://github.com/jblondin/csv-sniffer/issues/1"
issue_date = 2021-01-05
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

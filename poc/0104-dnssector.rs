/*!
```rudra-poc
[target]
crate = "dnssector"
version = "0.2.0"

[test]
analyzers = ["Manual", "UnsafeDataflow"]
bug_classes = ["InconsistencyAmplification"]

[report]
issue_url = "https://github.com/jedisct1/dnssector/issues/14"
issue_date = 2021-01-19
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
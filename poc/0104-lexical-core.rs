/*!
```rudra-poc
[target]
crate = "lexical-core"
version = "0.7.4"

[test]
analyzers = ["Manual", "UnsafeDataflow"]
bug_classes = ["InconsistencyAmplification", "Other"]

[report]
issue_url = "https://github.com/Alexhuszagh/rust-lexical/issues/53"
issue_date = 2021-01-08
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

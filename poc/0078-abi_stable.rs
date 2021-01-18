/*!
```rudra-poc
[target]
crate = "abi_stable"
version = "0.9.0"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["PanicSafety"]

[report]
issue_url = "https://github.com/rodrimati1992/abi_stable_crates/issues/44"
issue_date = 2020-12-21
unique_bugs = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

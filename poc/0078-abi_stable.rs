/*!
```rudra-poc
[target]
crate = "abi_stable"
version = "0.9.0"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/rodrimati1992/abi_stable_crates/issues/44"
issue_date = 2020-12-21
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

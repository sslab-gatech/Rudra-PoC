/*!
```rudra-poc
[target]
crate = "abi_stable"
version = "0.9.0"

[report]
issue_url = "https://github.com/rodrimati1992/abi_stable_crates/issues/44"
issue_date = 2020-12-21
rustsec_url = "https://github.com/RustSec/advisory-db/pull/609"
rustsec_id = "RUSTSEC-2020-0105"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

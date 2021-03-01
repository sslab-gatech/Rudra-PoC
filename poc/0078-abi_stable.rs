/*!
```rudra-poc
[target]
crate = "abi_stable"
version = "0.9.0"
indexed_version = "0.8.3"

[report]
issue_url = "https://github.com/rodrimati1992/abi_stable_crates/issues/44"
issue_date = 2020-12-21
rustsec_url = "https://github.com/RustSec/advisory-db/pull/609"
rustsec_id = "RUSTSEC-2020-0105"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 2
rudra_report_locations = [
    "src/std_types/vec/iters.rs:294:5: 312:6",
    "src/std_types/string.rs:613:5: 646:6",
]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

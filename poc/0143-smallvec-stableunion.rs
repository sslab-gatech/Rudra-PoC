/*!
```rudra-poc
[target]
crate = "smallvec-stableunion"
version = "0.6.10"

[report]
issue_date = 2021-02-19
issue_url = "https://github.com/ColonelPhantom/rust-smallvec/pull/1"

[[bugs]]
analyzer = "UnsafeDataflow"
guide = "Manual"
bug_class = "Other"
rudra_report_locations = ["lib.rs:751:5: 793:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("Reported without PoC as duplicate of smallvec 0103")
}
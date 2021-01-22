/*!
```rudra-poc
[target]
crate = "basic_dsp_matrix"
version = "0.9.0"

[report]
issue_url = "https://github.com/liebharc/basic_dsp/issues/47"
issue_date = 2021-01-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/607"
rustsec_id = "RUSTSEC-2021-0009"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 6
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

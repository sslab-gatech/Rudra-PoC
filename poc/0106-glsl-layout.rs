/*!
```rudra-poc
[target]
crate = "glsl-layout"
version = "0.3.2"

[report]
issue_url = "https://github.com/rustgd/glsl-layout/pull/10"
issue_date = 2021-01-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/568"
rustsec_id = "RUSTSEC-2021-0005"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/array.rs:70:13: 105:14"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    // The reported issue is a potential double-drop bug.
    // The author was already aware of the issue,
    // so I submitted a fix PR without creating a PoC.
    panic!("This issue was reported without PoC");
}

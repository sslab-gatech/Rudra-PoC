/*!
```rudra-poc
[target]
crate = "pulse-simple"
version = "1.0.1"

[report]
issue_url = "https://github.com/astro/rust-pulse-simple/issues/5"
issue_date = 2021-02-05

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "InconsistencyAmplification"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
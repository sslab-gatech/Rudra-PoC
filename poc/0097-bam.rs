/*!
```rudra-poc
[target]
crate = "bam"
version = "0.1.2"

[report]
issue_url = "https://gitlab.com/tprodanov/bam/-/issues/4"
issue_date = 2021-01-07

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

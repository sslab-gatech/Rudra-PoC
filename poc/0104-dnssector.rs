/*!
```rudra-poc
[target]
crate = "dnssector"
version = "0.2.0"

[report]
issue_url = "https://github.com/jedisct1/dnssector/issues/14"
issue_date = 2021-01-19

[[bugs]]
analyzer = "UnsafeDataflow"
guide = "Manual"
bug_class = "InconsistencyAmplification"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
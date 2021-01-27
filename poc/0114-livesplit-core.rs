/*!
```rudra-poc
[target]
crate = "livesplit-core"
version = "0.11.0"

[report]
issue_url = "https://github.com/LiveSplit/livesplit-core/issues/400"
issue_date = 2021-01-26

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

/*!
```rudra-poc
[target]
crate = "acc_reader"
version = "2.0.0"

[report]
issue_url = "https://github.com/netvl/acc_reader/issues/1"
issue_date = 2020-12-27

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

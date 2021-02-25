/*!
```rudra-poc
[target]
crate = "bayer"
version = "0.1.5"

[report]
issue_url = "https://github.com/wangds/libbayer/issues/1"
issue_date = 2021-02-25

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "Other"
rudra_report_locations = ["src/ffi.rs:32:1: 74:2"]
```
!*/
#![forbid(unsafe_code)]

use bayer;

fn main() {
    panic!("Issue reported without PoC");
}
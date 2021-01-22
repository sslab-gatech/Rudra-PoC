/*!
```rudra-poc
[target]
crate = "rusb"
version = "0.6.5"

[report]
issue_url = "https://github.com/a1ien/rusb/issues/44"
issue_date = 2020-12-18
rustsec_url = "https://github.com/RustSec/advisory-db/pull/580"
rustsec_id = "RUSTSEC-2020-0098"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 4
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

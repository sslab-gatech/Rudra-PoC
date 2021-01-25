/*!
```rudra-poc
[target]
crate = "libsbc"
version = "0.1.4"

[[target.peer]]
crate = "static_assertions"
version = "1.1.0"

[report]
issue_url = "https://github.com/mvertescher/libsbc-rs/issues/4"
issue_date = 2020-11-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/679"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
```
!*/
#![forbid(unsafe_code)]

use libsbc::Decoder;

use static_assertions::{assert_impl_all, assert_not_impl_all};
use std::io;

fn main() {
    assert_not_impl_all!(io::StdinLock: Send);
    assert_not_impl_all!(Decoder<io::StdinLock> : Send);
}

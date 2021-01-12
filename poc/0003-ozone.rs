/*!
```rudra-poc
[target]
crate = "ozone"
version = "0.1.0"

[test]
analyzers = ["Manual"]
bug_classes = ["Other"]

[report]
issue_date = 2020-07-04
rustsec_url = "https://github.com/RustSec/advisory-db/pull/328"
rustsec_id = "RUSTSEC-2020-0022"
```
!*/
#![forbid(unsafe_code)]

use ozone::SwapBackedHashMap;

fn main() {
    let mut map = SwapBackedHashMap::new();
    map.insert(0, 0);
}

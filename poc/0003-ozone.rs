/*!
```crux-poc
[target]
crate = "ozone"
version = "0.1.0"

[test]
analyzers = ["manual"]

[report]
title = "Ozone contains several memory safety issues including out-of-bound access and dropping of uninitialized memory."
description = """
Ozone contains several memory safety issues including [out-of-bound access](https://github.com/bqv/ozone/blob/e21f948b0178ab305f644118f18d87a838c618e0/src/buffer.rs#L38-L48)
and dropping of [uninitialized memory](https://github.com/bqv/ozone/blob/e21f948b0178ab305f644118f18d87a838c618e0/src/map.rs#L94-L101)."""
code_snippets = []
patched = []
issue_date = 2020-07-04
rustsec_url = "https://github.com/RustSec/advisory-db/pull/328"
rustsec_id = "RUSTSEC-2020-0022"
```
!*/

use ozone::SwapBackedHashMap;

fn main() {
    let mut map = SwapBackedHashMap::new();
    map.insert(0, 0);
}

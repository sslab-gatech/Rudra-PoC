/*!
```rudra-poc
[target]
crate = "libsbc"
version = "0.1.4"

[[target.peer]]
crate = "static_assertions"
version = "1.1.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "libsbc lacks a Send constraint on its Send trait"
description = """
"""
code_snippets = ["https://github.com/mvertescher/libsbc-rs/blob/7278b23901f93d956d9739fdfc4ced147cc3f242/src/lib.rs#L34-L38"]
patched = []
informational = "unsound"
issue_url = "https://github.com/mvertescher/libsbc-rs/issues/4"
issue_date = 2020-11-10
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

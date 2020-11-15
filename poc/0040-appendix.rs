/*!
```crux-poc
[target]
crate = "appendix"
version = "0.2.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "issue title"
description = """
issue description"""
code_snippets = ["https://github.com/krl/appendix/blob/b279bc230bfd5df4695c7a095f687d5c3a184e97/src/lib.rs#L61-L62"]
patched = []
informational = "unsound"
issue_url = "https://github.com/krl/appendix/issues/6"
issue_date = 2020-11-15
```
!*/
#![forbid(unsafe_code)]

use appendix::Index;

use std::env;
use std::fs;

fn main() {
    // Set up a temporary directory for the index.
    let tmp_dir = env::temp_dir().join("appendix-index");
    fs::remove_dir(&tmp_dir);
    fs::create_dir(&tmp_dir);

    let index = Index::new(&tmp_dir).unwrap();
    index.insert(0, &10).unwrap();
}

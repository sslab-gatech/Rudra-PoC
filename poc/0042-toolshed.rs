/*!
```crux-poc
[target]
crate = "toolshed"
version = "0.8.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "CopyCell<T> implements Send for all types potentially allowing for data races"
description = """
issue description"""
code_snippets = []
patched = []
informational = "unsound"
issue_url = "https://github.com/ratel-rust/toolshed/issues/12"
issue_date = 2020-11-15
```
!*/
#![forbid(unsafe_code)]

use toolshed::CopyCell;

use std::cell::Cell;
use crossbeam_utils::thread;

fn main() {
    let cell = Cell::new(42);
    let copy_cell = CopyCell::new(&cell);

    thread::scope(|s| {
        s.spawn(move |_| {
            let smuggled_cell_ref = copy_cell.get();
            println!("Other Thread: {:p}", smuggled_cell_ref);
        });

        println!("Main Thread:  {:p}", &cell);
    });
}

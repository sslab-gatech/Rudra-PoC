/*!
```rudra-poc
[target]
crate = "toolshed"
version = "0.8.1"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[report]
issue_url = "https://github.com/ratel-rust/toolshed/issues/12"
issue_date = 2020-11-15
rustsec_url = "https://github.com/RustSec/advisory-db/pull/591"
rustsec_id = "RUSTSEC-2020-0136"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
rudra_report_locations = ["src/cell.rs:22:1: 22:39"]
```
!*/
#![forbid(unsafe_code)]

use toolshed::CopyCell;

use crossbeam_utils::thread;
use std::cell::Cell;

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

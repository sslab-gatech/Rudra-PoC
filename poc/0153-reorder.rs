/*!
```rudra-poc
[target]
crate = "reorder"
version = "1.0.3"

[report]
issue_url = "https://github.com/tiby312/reorder/issues/1"
issue_date = 2021-02-24

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "InconsistencyAmplification"
rudra_report_locations = ["src/lib.rs:45:1: 57:2"]
```
!*/
#![forbid(unsafe_code)]

use reorder::swap_index;

struct IteratorWithWrongLength();

impl Iterator for IteratorWithWrongLength {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> { None }
}
impl ExactSizeIterator for IteratorWithWrongLength {
    fn len(&self) -> usize { 1024 }
}

fn main() {
    let v = swap_index(IteratorWithWrongLength());

    println!("{:?}", v);
}
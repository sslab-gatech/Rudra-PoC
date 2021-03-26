/*!
```rudra-poc
[target]
crate = "insert_many"
version = "0.1.1"

[report]
issue_url = "https://github.com/rphmeier/insert_many/issues/1"
issue_date = 2021-01-26
rustsec_url = "https://github.com/RustSec/advisory-db/pull/832"
rustsec_id = "RUSTSEC-2021-0042"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 2
rudra_report_locations = ["src/lib.rs:52:5: 54:6", "src/lib.rs:59:5: 61:6"]
```
!*/
#![forbid(unsafe_code)]

use insert_many::InsertMany;

struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

// A type with an iterator that panics.
// -----
struct MyCollection();

impl IntoIterator for MyCollection {
    type Item = DropDetector;
    type IntoIter = PanickingIterator;

    fn into_iter(self) -> Self::IntoIter { PanickingIterator() }
}

struct PanickingIterator();

impl Iterator for PanickingIterator {
    type Item = DropDetector;

    fn next(&mut self) -> Option<Self::Item> { panic!("Iterator panicked"); }
}

impl ExactSizeIterator for PanickingIterator {
    fn len(&self) -> usize { 1 }
}
// -----


fn main() {
    let mut v = vec![DropDetector(1), DropDetector(2)];

    // Inserting many elements from a panicking iterator will cause a double-drop.
    v.insert_many(0, MyCollection());
}
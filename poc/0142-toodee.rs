/*!
```rudra-poc
[target]
crate = "toodee"
version = "0.2.1"

[report]
issue_date = 2021-02-19
issue_url = "https://github.com/antonmarsden/toodee/issues/13"
rustsec_url = "https://github.com/RustSec/advisory-db/pull/784"
rustsec_id = "RUSTSEC-2021-0028"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ['src/toodee.rs:561:5: 590:6']

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "InconsistencyAmplification"
rudra_report_locations = ['src/toodee.rs:561:5: 590:6']
```
!*/
#![forbid(unsafe_code)]

use toodee::TooDee;

struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

// An iterator that panics.
// -----
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
    reserves_based_on_iterator_length();

    //let vec = vec![DropDetector(1), DropDetector(2), DropDetector(3)];
    //let mut toodee : TooDee<_> = TooDee::from_vec(1, 3, vec);

    //toodee.insert_row(0, PanickingIterator());
}


struct IteratorWithWrongLength();

impl Iterator for IteratorWithWrongLength {
    type Item = Box<u8>;

    fn next(&mut self) -> Option<Self::Item> { None }
}

impl ExactSizeIterator for IteratorWithWrongLength {
    fn len(&self) -> usize { 1 }
}

fn reserves_based_on_iterator_length() {
    let vec = vec![Box::<u8>::new(1)];
    let mut toodee : TooDee<_> = TooDee::from_vec(1, 1, vec);

    toodee.insert_row(1, IteratorWithWrongLength());

    println!("{}", toodee[1][0]);
}

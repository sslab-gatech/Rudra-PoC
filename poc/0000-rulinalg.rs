/*!
```rudra-poc
[target]
crate = "rulinalg"
version = "0.4.2"

[test]
analyzers = ["Manual"]
bug_classes = ["Other"]

[report]
issue_url = "https://github.com/AtheMathmo/rulinalg/issues/201"
issue_date = 2020-02-11
rustsec_url = "https://github.com/RustSec/advisory-db/pull/319"
rustsec_id = "RUSTSEC-2020-0023"
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

use rulinalg::matrix;
use rulinalg::matrix::BaseMatrixMut;

fn main() {
    let mut mat = matrix![0];

    let mut row = mat.row_mut(0);

    // this creates mutable aliases to the same location
    let raw_slice1 = row.raw_slice_mut();
    let raw_slice2 = row.raw_slice_mut();

    assert_eq!(raw_slice1[0], 0);
    raw_slice2[0] = 1;
    assert_eq!(raw_slice1[0], 0);
}

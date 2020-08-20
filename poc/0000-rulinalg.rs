/*!
```crux-poc
[target]
crate = "rulinalg"
version = "0.4.2"

[test]
analyzers = ["manual"]

[report]
title = "Lifetime boundaries for `raw_slice` and `raw_slice_mut` are incorrect"
description = """
The current definition of `raw_slice()` and `raw_slice_mut()` creates `'a` bounded reference from `&self`.
Since the returned slice is created from a stored pointer in `&self`,
it should be bounded by `'self` lifetime instead of `'a`.

The current definition allows safe Rust code to trigger data race."""
code_snippets = []
patched = []
issue_url = "https://github.com/AtheMathmo/rulinalg/issues/201"
issue_date = 2020-07-29
rustsec_url = "https://github.com/RustSec/advisory-db/pull/319"
rustsec_id = "RUSTSEC-2020-0023"
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

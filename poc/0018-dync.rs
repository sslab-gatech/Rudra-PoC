/*!
```crux-poc
[target]
crate = "dync"
version = "0.4.0"

[test]
analyzers = ["manual"]

[report]
title = "VecCopy allows misaligned access to elements"
description = """
`VecCopy::data` is created as a Vec of u8 but can be used to store and retrieve
elements of different types leading to misaligned access."""
code_snippets = []
patched = []
informational = "unsound"
issue_url = "https://github.com/elrnv/dync/issues/4"
issue_date = 2020-09-27
```
!*/
#![forbid(unsafe_code)]

mod boilerplate;

use dync::{VecCopy, VTable};

#[repr(align(256))]
#[derive(Copy, Clone)]
struct LargeAlign(u8);

impl VTable<LargeAlign> for LargeAlign {
    fn build_vtable() -> Self {
        LargeAlign(0)
    }
}

fn main() {
    // The backing storage for a VecCopy is a u8, meaning that casting to a type
    // with different alginment requires triggers undefined behavior.
    // https://github.com/elrnv/dync/blob/c133056676582dd0e28c14526175d0c9ae01a905/src/vec_copy.rs#L64-L65
    let mut x = VecCopy::<LargeAlign>::with_type();
    x.push_as::<LargeAlign>(LargeAlign(0));

    let ref_to_element = x.get_ref_as::<LargeAlign>(0).unwrap();
    boilerplate::assert_aligned(ref_to_element);
}

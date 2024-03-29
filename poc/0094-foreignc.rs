/*!
```rudra-poc
[target]
crate = "foreignc"
version = "0.1.2"

[report]
issue_url = "https://github.com/mart368b/foreignc/issues/1"
issue_date = 2021-01-06

[[bugs]]
analyzer = "UnsafeDataflow"
guide = "Manual"
bug_class = "Other"
rudra_report_locations = ["src/ffi_util.rs:77:5: 84:6"]


[[bugs]]
analyzer = "Manual"
guide = "UnsafeDataflow"
bug_class = "Other"
rudra_report_locations = []
```
!*/
#![forbid(unsafe_code)]

use foreignc::{FFiResult, FromFFi};

use std::mem::MaybeUninit;

fn main() {
    let mut x: i8 = 0x61;
    let a: i8 = 0x66;
    let mut uninit: MaybeUninit<i16> = MaybeUninit::uninit();

    let ptr = &mut x as *mut i8 as *mut i64;
    let b: FFiResult<Option<i64>> = FromFFi::<*mut i64>::from_ffi(ptr);
    let z: i64 = b.unwrap().unwrap();

    println!("0x{:X}, 0x{:X}", z, a);
    panic!("We can read from uninitialized memory using safe `from_ffi` API");
}

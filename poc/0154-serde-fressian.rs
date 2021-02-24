/*!
```rudra-poc
[target]
crate = "serde-fressian"
version = "0.1.1"

[report]
issue_url = "https://github.com/pkpkpk/serde-fressian/issues/1"
issue_date = 2021-02-24

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "Other"
rudra_report_locations = ["src/wasm/mod.rs:72:1: 81:2"]
```
!*/
#![forbid(unsafe_code)]

use serde_fressian::wasm::from_ptr;

fn return_raw_pointer() -> *mut u8 {
    let mut array: [u8; 4] = [0x41, 0x42, 0x43, 0x44];
    array.as_mut_ptr()
}

fn main() {
    let raw_ptr = return_raw_pointer();
    let deserialized : i32 = from_ptr(raw_ptr, 4).unwrap();

    println!("{:x}", deserialized);
    assert!(deserialized == 0x41424344)
}
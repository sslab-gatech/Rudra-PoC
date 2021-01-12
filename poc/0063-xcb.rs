/*!
```rudra-poc
[target]
crate = "xcb"
version = "0.9.0"

[test]
analyzers = ["Manual"]

[report]
issue_date = 2020-12-10
issue_url = "https://github.com/rtbo/rust-xcb/issues/93"
```
!*/
#![forbid(unsafe_code)]

use xcb::base::Error;

fn main() {
    let mut v1: Vec<i8> = vec![1, 2, 3, 0];
    let _ = Error {
        ptr: v1.as_mut_ptr(),
    };

    // use-after-free in v1
    // v1 and v2 are now backed by the same buffer.
    let v2: Vec<i8> = vec![4, 5, 6, 0];

    let measure1 = v2[0];
    v1[0] = 123;
    let measure2 = v2[0];

    assert_eq!(measure1, measure2);
}

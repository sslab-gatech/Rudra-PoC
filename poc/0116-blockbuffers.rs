/*!
```rudra-poc
[target]
crate = "blockbuffers"
version = "0.1.0"

[report]

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
```
!*/
#![forbid(unsafe_code)]

use blockbuffers::le::LE;

use std::mem::size_of;

struct MyCustomLEType {
    val: Box<u8>,
}

impl LE for MyCustomLEType {
    fn to_le(self) -> Self {
        self
    }
    fn from_le(x: Self) -> Self {
        x
    }
}

fn main() {
    let custom_type = MyCustomLEType::from_le_slice(&[0; size_of::<MyCustomLEType>()]);
    println!("{}", custom_type.val);
}

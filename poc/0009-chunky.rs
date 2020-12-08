/*!
```rudra-poc
[target]
crate = "chunky"
version = "0.3.7"

[test]
analyzers = ["manual", "UnsafeDestructor"]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/aeplay/chunky/issues/2"
issue_date = 2020-08-25
rustsec_url = "https://github.com/RustSec/advisory-db/pull/366"
rustsec_id = "RUSTSEC-2020-0035"
```
!*/
#![forbid(unsafe_code)]

use chunky::{HeapStorage, Ident, Value};
use std::rc::Rc;

#[repr(align(256))]
struct LargeAlign(u8);

impl Drop for LargeAlign {
    fn drop(&mut self) {
        println!("Dropped");
    }
}

fn main() {
    let ident = Ident(String::from("ident"));
    let storage = Rc::new(HeapStorage);
    let value = Value::load_or_default(ident, LargeAlign(0), storage.clone());

    // Value reference does not have a correct alignment
    let v = &*value as *const _ as usize;
    println!("{:x}", v);
    assert!(v % std::mem::align_of::<LargeAlign>() == 0);

    // https://github.com/aeplay/chunky/blob/ef8533aec961eb5f415414dcd81ec4b395bae177/src/value.rs#L43-L49
    // Another bug that is not UB: `LargeAlign::Drop` is not called due to incorrect `drop_in_place()` in `Value::drop()`.
    // "Dropped" should be printed otherwise.
}

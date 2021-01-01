/*!
```rudra-poc
[target]
crate = "array_iterator"
version = "1.2.0"

[test]
analyzers = ["manual", "PanicSafety"]

[report]
issue_url = "https://gitlab.com/kevincox/array_iterator.rs/-/issues/1"
issue_date = 2020-12-31
```
!*/
#![forbid(unsafe_code)]
use array_iterator::{Array, ArrayIterator};

use std::mem::MaybeUninit;

struct MyArr(Vec<String>);

// Slightly modified from
// https://docs.rs/array_iterator/1.2.0/src/array_iterator/lib.rs.html#134-155
impl Array<String> for MyArr {
    type MaybeUninit = Vec<MaybeUninit<String>>;
    
    fn into_maybeunint(self) -> Self::MaybeUninit {
        debug_assert_eq!(std::mem::size_of::<Self::MaybeUninit>(), std::mem::size_of::<Self>());
        let mut partial: Self::MaybeUninit = unsafe { std::mem::transmute_copy(&self.0) };
        std::mem::forget(self);

        // DANGEROUS! Appending uninitialized objects to iterator..
        for _ in 0..10 {
            partial.push(MaybeUninit::uninit());
        }
        //
        partial
    }
}

fn main() {
    let my_arr = MyArr(vec![
        String::from("Hello"),
        String::from("World")
    ]);
    for x in ArrayIterator::new(my_arr) {
        println!("{} {:?}, {}", x.len(), x.as_bytes(), x);

        // SEGFAULT within this loop
    }

    panic!(
        "In DEBUG mode,\n
        this panic was unreachable when tested with rustc 1.48.0 on Ubuntu 18.04"
    );
}

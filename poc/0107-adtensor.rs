/*!
```rudra-poc
[target]
crate = "adtensor"
version = "0.0.3"

[[target.peer]]
crate = "typenum"
version = "1.12.0"

[[target.peer]]
crate = "generic-array"
version = "0.14.4"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/charles-r-earp/adtensor/issues/4"
issue_date = 2021-01-11
```
!*/
#![forbid(unsafe_code)]
// tested with `rustc 1.47.0-nightly (bf4342114 2020-08-25)`
use adtensor::vector::Vector;
use std::iter::{FromIterator, Iterator, repeat};
use typenum::U40;
use generic_array::{GenericArray, ArrayLength};

fn main() {
    type T = String;
    let x = Vector::<T, U40>::from_iter(
        repeat(String::from("Hello World")).take(10)
    );

    panic!("Program will segfault before this point {:?}", x);
}
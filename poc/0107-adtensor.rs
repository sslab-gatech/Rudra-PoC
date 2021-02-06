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

[report]
issue_url = "https://github.com/charles-r-earp/adtensor/issues/4"
issue_date = 2021-01-11

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 2
rudra_report_locations = ["src/matrix.rs:49:3: 59:4", "src/vector.rs:66:3: 74:4"]
```
!*/
#![forbid(unsafe_code)]
// tested with `rustc 1.47.0-nightly (bf4342114 2020-08-25)`
use adtensor::vector::Vector;
use generic_array::{ArrayLength, GenericArray};
use std::iter::{repeat, FromIterator, Iterator};
use typenum::U40;

fn main() {
    type T = String;
    let x = Vector::<T, U40>::from_iter(repeat(String::from("Hello World")).take(10));

    panic!("Program will segfault before this point {:?}", x);
}

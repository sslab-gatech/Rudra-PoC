/*!
```rudra-poc
[target]
crate = "alg_ds"
version = "0.3.1"

[test]
analyzers = ["UnsafeDestructor", "PanicSafety"]
cargo_flags = ["--release"]

[report]
issue_url = "https://gitlab.com/dvshapkin/alg-ds/-/issues/1"
issue_date = 2020-08-25
rustsec_url = "https://github.com/RustSec/advisory-db/pull/362"
rustsec_id = "RUSTSEC-2020-0033"
```
!*/
#![forbid(unsafe_code)]

use alg_ds::ds::matrix::Matrix;
use std::sync::atomic::{AtomicUsize, Ordering};

static creation_cnt: AtomicUsize = AtomicUsize::new(0);
static drop_cnt: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
struct DropDetector(u32);

impl Default for DropDetector {
    fn default() -> Self {
        creation_cnt.fetch_add(1, Ordering::Relaxed);
        DropDetector(12345)
    }
}

impl Drop for DropDetector {
    fn drop(&mut self) {
        drop_cnt.fetch_add(1, Ordering::Relaxed);
        println!("Dropping {}", self.0);
    }
}

fn main() {
    // Please check along with the code snippets above.
    {
        // `*ptr = value` acts by dropping existing contents at `ptr`.
        // `Matrix::fill_with()` uses this pattern which result in dropping
        // uninitialized, unallocated struct.
        //
        // Note that the creation of a mutable reference to uninitialized memory
        // region is already UB by itself.
        // `ptr::write` and `MaybeUninit` should be used for the initialization.
        let _ = Matrix::<DropDetector>::new(1, 1);
    }
    {
        // (Bonus) Integer overflow in `layout()` allows to create a huge matrix.
        // Fortunately, every access to the internal buffer are bound-checked,
        // so this doesn't lead to obvious UB by itself.
        let mat = Matrix::<usize>::new(15326306685794188004, 0x123456789);
        println!(
            "rows: {}, cols: {}, number of elements: {}",
            mat.rows(),
            mat.cols(),
            mat.elements_number()
        );
    }
    assert_eq!(
        creation_cnt.load(Ordering::Relaxed),
        drop_cnt.load(Ordering::Relaxed)
    );
}

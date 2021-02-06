/*!
```rudra-poc
[target]
crate = "rblas"
version = "0.0.13"

[report]
issue_date = 2021-02-05
issue_url = "https://github.com/mikkyang/rust-blas/issues/26"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 3
rudra_report_locations = [
    "src/vector/mod.rs:30:5: 38:6",
    "src/math/matrix_vector.rs:21:5: 31:6",
    "src/math/mat.rs:111:5: 125:6",
]
```
!*/
#![forbid(unsafe_code)]

use rblas::matrix::Matrix;
use rblas::vector::ops::Copy;
use rblas::vector::Vector;

#[derive(Clone, Debug)]
struct MyType(Box<u64>);

impl Copy for MyType {
    fn copy<V, W>(src: &V, dst: &mut W)
    where
        V: ?Sized + Vector<Self>,
        W: ?Sized + Vector<Self>,
    {
    }

    fn copy_mat(src: &Matrix<Self>, dst: &mut Matrix<Self>) {}
}

fn main() {
    let vec = vec![MyType(Box::new(42))];
    let as_blas_vector = &vec as &Vector<_>;

    let back_to_vec: Vec<MyType> = as_blas_vector.into();
}

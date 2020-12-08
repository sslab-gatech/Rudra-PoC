/*!
```rudra-poc
[target]
crate = "failure"
version = "0.1.8"

[test]
analyzers = ["manual"]

[report]
issue_url = "https://github.com/rust-lang-nursery/failure/issues/336"
issue_date = 2019-11-13
rustsec_url = "https://github.com/RustSec/advisory-db/pull/318"
rustsec_id = "RUSTSEC-2019-0036"
```
!*/
#![forbid(unsafe_code)]

use std::any::TypeId;
use std::fmt::{self, Display};

use failure::Fail;

#[derive(Debug)]
struct Error1 {
    name: String,
}

impl Display for Error1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error1 ({})", self.name)
    }
}

impl Fail for Error1 {
    fn __private_get_type_id__(&self) -> TypeId {
        TypeId::of::<Error2>()
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Error2")]
struct Error2 {
    p1: usize,
    p2: usize,
    p3: usize,
}

fn main() {
    let e1: Box<dyn Fail> = Box::new(Error1 {
        name: "test".to_owned(),
    });
    let e2: Option<&Error2> = e1.downcast_ref();
    dbg!(e2);
}

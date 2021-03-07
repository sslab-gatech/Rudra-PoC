/*!
```rudra-poc
[target]
crate = "endian_trait"
version = "0.6.0"

[report]
issue_url = "https://gitlab.com/myrrlyn/endian_trait/-/issues/1"
issue_date = 2021-01-04
rustsec_url = "https://github.com/RustSec/advisory-db/pull/814"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 4
rudra_report_locations = ["src/slices.rs:9:2: 14:3", "src/slices.rs:15:2: 20:3", "src/slices.rs:21:2: 26:3", "src/slices.rs:27:2: 32:3"]
```
!*/
#![forbid(unsafe_code)]
use endian_trait::Endian;

#[derive(Debug)]
struct Foo(Box<Option<i32>>);

impl Endian for Foo {
    fn to_be(self) -> Self {
        println!("PANIC BY MISTAKE {}", self.0.as_ref().as_ref().unwrap());
        self
    }
    fn to_le(self) -> Self {
        self
    }
    fn from_be(self) -> Self {
        self
    }
    fn from_le(self) -> Self {
        self
    }
}

fn main() {
    let mut foo = [Foo(Box::new(None))];
    let x = (&mut foo).to_be();
    dbg!(x);
}

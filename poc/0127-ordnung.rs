/*!
```rudra-poc
[target]
crate = "ordnung"
version = "0.0.1"

[report]
issue_date = 2020-09-03
issue_url = "https://github.com/maciejhirsz/ordnung/issues/8"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
```
!*/
#![forbid(unsafe_code)]

use ordnung::compact::Vec;

struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let mut vec = Vec::new();
    vec.push(DropDetector(1));

    vec.remove(999);
}
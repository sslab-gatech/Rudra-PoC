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

fn main() {
    println!("Hello, World!")
}
/*!
```rudra-poc
[target]
crate = "through"
version = "0.1.0"

[report]
issue_url = "https://github.com/gretchenfrage/through/issues/1"
issue_date = 2021-02-18

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 2
rudra_report_locations = ["src/lib.rs:5:1: 12:2", "src/lib.rs:16:1: 24:2"]
```
!*/
#![forbid(unsafe_code)]
use through::through;

fn main() {
    let mut hello = String::from("Hello");
    let object = through(&mut hello, |mut s| {
        s.push_str(" World!");
        panic!("Unexpected panic");
        s
    });
    dbg!(object);
}

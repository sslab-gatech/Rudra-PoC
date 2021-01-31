/*!
```rudra-poc
[target]
crate = "postscript"
version = "0.13.2"

[report]
issue_url = "https://github.com/bodoni/postscript/issues/1"
issue_date = 2021-01-30

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

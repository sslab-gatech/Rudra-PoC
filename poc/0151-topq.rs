/*!
```rudra-poc
[target]
crate = "topq"
version = "0.2.0"

[report]
issue_url = "https://github.com/jamesmunns/topq/issues/1"
issue_date = 2021-02-24

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 2
rudra_report_locations = ["src/lib.rs:97:5: 143:6", "src/lib.rs:148:5: 183:6"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("Issue reported without PoC");
}
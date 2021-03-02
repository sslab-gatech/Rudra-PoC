/*!
```rudra-poc
[target]
crate = "serde-gff"
version = "0.3.0"

[report]
issue_url = "https://github.com/Mingun/serde-gff/issues/2"
issue_date = 2021-03-02

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
rudra_report_locations = ["src/parser/mod.rs:248:3: 257:4", "src/parser/mod.rs:336:3: 343:4", "src/raw.rs:315:3: 337:4"]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("Issue reported without PoC");
}

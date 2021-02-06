/*!
```rudra-poc
[target]
crate = "messagepack-rs"
version = "0.8.0"

[report]
issue_url = "https://github.com/otake84/messagepack-rs/issues/2"
issue_date = 2021-01-26

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 4
rudra_report_locations = [
    "src/deserializable.rs:59:5: 64:6",
    "src/deserializable.rs:66:5: 71:6",
    "src/deserializable.rs:130:5: 135:6",
    "src/deserializable.rs:82:9: 93:10",
]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

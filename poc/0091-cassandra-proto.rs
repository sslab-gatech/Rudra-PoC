/*!
```rudra-poc
[target]
crate = "cassandra-proto"
version = "0.1.2"

[report]
issue_url = "https://github.com/AlexPikalov/cassandra-proto/issues/3"
issue_date = 2021-01-05

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

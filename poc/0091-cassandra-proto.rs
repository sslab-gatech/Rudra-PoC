/*!
```rudra-poc
[target]
crate = "cassandra-proto"
version = "0.1.2"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/AlexPikalov/cassandra-proto/issues/3"
issue_date = 2021-01-05
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}
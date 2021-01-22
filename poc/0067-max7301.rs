/*!
```rudra-poc
[target]
crate = "max7301"
version = "0.1.0"

[report]
issue_url = "https://github.com/edarc/max7301/issues/1"
issue_date = 2020-12-18

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

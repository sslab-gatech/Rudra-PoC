/*!
```rudra-poc
[target]
crate = "appendix"
version = "0.2.0"

[report]
issue_url = "https://github.com/krl/appendix/issues/6"
issue_date = 2020-11-15

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/lib.rs:61:1: 61:42", "src/lib.rs:62:1: 62:42"]
```
!*/
#![forbid(unsafe_code)]

use appendix::Index;

use std::env;
use std::fs;

fn main() {
    // Set up a temporary directory for the index.
    let tmp_dir = env::temp_dir().join("appendix-index");
    fs::remove_dir(&tmp_dir);
    fs::create_dir(&tmp_dir);

    let index = Index::new(&tmp_dir).unwrap();
    index.insert(0, &10).unwrap();
}

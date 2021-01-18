/*!
```rudra-poc
[target]
crate = "tensorflow"
version = "0.15.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance", "InconsistencyAmplification"]

[report]
issue_url = "https://github.com/tensorflow/rust/issues/284"
issue_date = 2020-12-08
unique_bugs = 1
additional_send_sync_violations = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

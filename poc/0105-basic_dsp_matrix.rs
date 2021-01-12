/*!
```rudra-poc
[target]
crate = "basic_dsp_matrix"
version = "0.9.0"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["PanicSafety"]

[report]
issue_url = "https://github.com/liebharc/basic_dsp/issues/47"
issue_date = 2021-01-10
```
!*/
#![forbid(unsafe_code)]

fn main() {
    panic!("This issue was reported without PoC");
}

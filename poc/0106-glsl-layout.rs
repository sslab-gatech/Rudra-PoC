/*!
```rudra-poc
[target]
crate = "glsl-layout"
version = "0.3.2"

[test]
analyzers = ["PanicSafety"]

[report]
issue_url = "https://github.com/rustgd/glsl-layout/pull/10"
issue_date = 2021-01-10
```
!*/
#![forbid(unsafe_code)]

fn main() {
    // The reported issue is a potential double-drop bug.
    // The author was already aware of the issue,
    // so I submitted a fix PR without creating a PoC.
    panic!("This issue was reported without PoC");
}
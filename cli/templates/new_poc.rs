/*!
```rudra-poc
[target]
crate = "{{ krate }}"
version = "{{ version }}"

[test]
analyzers = []
bug_classes = []

[report]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    println!("Hello, World!")
}

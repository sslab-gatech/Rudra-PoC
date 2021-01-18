/*!
```rudra-poc
[target]
crate = "{{ krate }}"
version = "{{ version }}"

[test]
analyzers = []
bug_classes = []

[report]
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

fn main() {
    println!("Hello, World!")
}

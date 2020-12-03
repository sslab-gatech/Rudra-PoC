/*!
```rudra-poc
[target]
crate = "{{ krate }}"
version = "{{ version }}"

[test]
analyzers = []

[report]
```
!*/
#![forbid(unsafe_code)]

fn main() {
    println!("Hello, World!")
}

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

mod boilerplate;

fn main() {
    boilerplate::init();

    println!("Hello, World!")
}

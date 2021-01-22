/*!
```rudra-poc
[target]
crate = "{{ krate }}"
version = "{{ version }}"

[report]

[[bugs]]
analyzer = ""
bug_class = ""
```
!*/
#![forbid(unsafe_code)]

fn main() {
    println!("Hello, World!")
}

/*!
```rudra-poc
[target]
crate = "algorithmica"
version = "0.1.8"

[report]
issue_url = "https://github.com/AbrarNitk/algorithmica/issues/1"
issue_date = 2021-03-07

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/sort/merge_sort.rs:9:1: 55:2"]
```
!*/
#![forbid(unsafe_code)]
use algorithmica::sort::merge_sort::sort;

fn main() {
    let mut arr = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("Rust"),
    ];

    // Calling `merge_sort::sort` on an array of `T: Drop` triggers double drop
    algorithmica::sort::merge_sort::sort(&mut arr);
    dbg!(arr);
}

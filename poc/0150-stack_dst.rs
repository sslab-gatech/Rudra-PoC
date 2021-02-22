/*!
```rudra-poc
[target]
crate = "stack_dst"
version = "0.6.0"

[report]
issue_url = "https://github.com/thepowersgang/stack_dst-rs/issues/5"
issue_date = 2021-02-22

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/stack.rs:184:2: 202:3"]
```
!*/
#![forbid(unsafe_code)]

use stack_dst::StackA;

#[derive(Debug)]
struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

impl Clone for DropDetector {
    fn clone(&self) -> Self { panic!("Panic in clone()") }
}

fn main() {
    let mut stack = StackA::<[DropDetector], [usize; 9]>::new();
    stack.push_stable([DropDetector(1)], |p| p).unwrap();
    stack.push_stable([DropDetector(2)], |p| p).unwrap();

    println!("Popping off second drop detector");
    let second_drop = stack.pop();

    println!("Pushing panicky-clone");
    stack.push_cloned(&[DropDetector(3)]).unwrap();
}
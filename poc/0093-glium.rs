/*!
```rudra-poc
[target]
crate = "glium"
version = "0.29.0"

[test]
analyzers = ["UnsafeDataflow"]
bug_classes = ["UninitExposure"]

[report]
issue_url = "https://github.com/glium/glium/issues/1907"
issue_date = 2021-01-06
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]
use glium::buffer::Content;

fn main() {
    let size = std::mem::size_of::<&i32>();
    let x = <[&i32; 1] as Content>::read(size, |x| {
        println!("ADDRESS: {:X}", x as *const _ as usize);
        println!("SEGFAULT HERE: {}", x[0]);
        Err(())
    });

    if x.is_err() {
        panic!("Program will segfault before reaching this point.");
    }
}

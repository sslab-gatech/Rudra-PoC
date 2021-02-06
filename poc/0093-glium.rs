/*!
```rudra-poc
[target]
crate = "glium"
version = "0.29.0"

[report]
issue_url = "https://github.com/glium/glium/issues/1907"
issue_date = 2021-01-06

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/buffer/mod.rs:145:5: 154:6"]
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

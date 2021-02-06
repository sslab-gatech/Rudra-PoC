/*!
```rudra-poc
[target]
crate = "arenavec"
version = "0.1.1"

[report]
issue_url = "https://github.com/ibabushkin/arenavec/issues/1"
issue_date = 2021-01-12

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 3
rudra_report_locations = [
    "src/common.rs:75:5: 89:6",
    "src/common.rs:418:5: 443:6",
    "src/common.rs:446:5: 471:6",
]
```
!*/
#![forbid(unsafe_code)]
// tested with rustc 1.50.0-nightly (7f9c43cf9 2020-12-23) on Ubuntu 18.04
use arenavec::rc::{Arena, SliceVec};
use arenavec::ArenaBacking;
use std::sync::atomic::{
    AtomicBool,
    Ordering::SeqCst,
};

#[derive(Clone)]
struct Foo(usize, Option<u64>);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping {:?}", self.0);
        if self.0 == 1 && ATOMIC_TRUE.compare_and_swap(true, false, SeqCst) {
            println!("THIS WILL PANIC {:?}", self.1.as_ref().unwrap());
        }
    }
}

static ATOMIC_TRUE: AtomicBool = AtomicBool::new(true);
const DEFAULT_CAPACITY: usize = 4096 << 8;
fn main() {
    let arena = Arena::init_capacity(ArenaBacking::SystemAllocation, DEFAULT_CAPACITY).unwrap();

    let mut vec: SliceVec<Foo> = SliceVec::new(arena.inner());
    vec.push(Foo(0, Some(12)));
    vec.push(Foo(1, None));
    assert_eq!(vec.len(), 2);

    vec.resize(1, Foo(99, Some(78)));
}

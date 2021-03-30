/*!
```rudra-poc
[target]
crate = "slice-deque"
version = "0.3.0"

[report]
issue_date = 2021-02-19
issue_url = "https://github.com/gnzlbg/slice_deque/issues/90"
rustsec_url = "https://github.com/RustSec/advisory-db/pull/846"
rustsec_id = "RUSTSEC-2021-0047"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/lib.rs:3016:5: 3040:6"]
```
!*/
#![forbid(unsafe_code)]

use slice_deque::SliceDeque;

struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let mut deq = SliceDeque::new();
    deq.push_back(DropDetector(1));
    deq.push_back(DropDetector(2));
    deq.push_back(DropDetector(3));

    let drained = deq
        .drain_filter(|x| {
            if x.0 == 1 {
                true
            } else if x.0 == 2 {
                false
            } else {
                panic!("predicate panicked!");
            }
        })
        .collect::<SliceDeque<_>>();;
}
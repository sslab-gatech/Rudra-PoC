/*!
```rudra-poc
[target]
crate = "generator"
version = "0.6.23"
indexed_version = "0.6.21"

[report]
issue_url = "https://github.com/Xudong-Huang/generator-rs/issues/27"
issue_date = 2020-11-16
rustsec_url = "https://github.com/RustSec/advisory-db/pull/855"
rustsec_id = "RUSTSEC-2020-0151"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
rudra_report_locations = ["src/gen_impl.rs:27:1: 27:55"]
```
!*/
#![forbid(unsafe_code)]

use generator::Gn;
use std::{rc::Rc, thread};

fn main() {
    let rc = Rc::new(());

    let rc_clone = rc.clone();
    let mut generator = Gn::new_scoped(move |_| {
        return rc_clone;
    });

    let child = thread::spawn(move || {
        let smuggled_rc = generator.next().unwrap();

        println!("RC in thread: {:p}", smuggled_rc);
        for _ in 0..1000000000 {
            let x = smuggled_rc.clone();
        }
    });

    println!("RC in main: {:p}", rc);
    for _ in 0..1000000000 {
        let x = rc.clone();
    }

    child.join().unwrap();
    assert_eq!(Rc::strong_count(&rc), 2);
}

/*!
```rudra-poc
[target]
crate = "parc"
version = "1.0.1"

[test]
analyzers = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/hyyking/rustracts/pull/6"
issue_date = 2020-11-14
```
!*/
#![forbid(unsafe_code)]

use parc::ParentArc;
use std::rc::Rc;

fn main() {
    // `Rc` neither implements `Send` nor `Sync`.
    let parent = ParentArc::new(Rc::new(0));

    let mut children = vec![];
    for _ in 0..5 {
        let weak = ParentArc::downgrade(&parent);
        let child_thr = std::thread::spawn(move || {
            loop {
                // `weak` is moved into child thread.
                let child = weak.upgrade();
                match child {
                    Some(rc) => {
                        for _ in 0..2000 {
                            // `strong_count` of `rc`
                            // is updated by multiple threads without synchronization.
                            let _ = Rc::clone(rc.as_ref());
                        }
                        break;
                    }
                    None => continue,
                }
            }
        });
        children.push(child_thr);
    }
    for child_thr in children {
        child_thr.join().expect("Failed to join with child thread");
    }

    let rc = parent.block_into_inner();

    // if (`strong_count` > 1): indicates a memory leak
    assert_eq!(1, Rc::strong_count(&rc));
}

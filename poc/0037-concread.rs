/*!
```rudra-poc
[target]
crate = "concread"
version = "0.2.5"

[report]
issue_url = "https://github.com/kanidm/concread/issues/48"
issue_date = 2020-11-13
rustsec_url = "https://github.com/RustSec/advisory-db/pull/532"
rustsec_id = "RUSTSEC-2020-0092"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
```
!*/
#![forbid(unsafe_code)]
use concread::arcache::ARCache;

use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let non_sync_item = Rc::new(0); // neither `Send` nor `Sync`
    assert_eq!(Rc::strong_count(&non_sync_item), 1);

    let cache = ARCache::<i32, Rc<u64>>::new_size(5, 5);
    let mut writer = cache.write();
    writer.insert(0, non_sync_item);
    writer.commit();

    let arc_parent = Arc::new(cache);

    let mut handles = vec![];
    for _ in 0..5 {
        let arc_child = arc_parent.clone();
        let child_handle = std::thread::spawn(move || {
            let reader = arc_child.read(); // new Reader of ARCache
            let smuggled_rc = reader.get(&0).unwrap();

            for _ in 0..1000 {
                let _dummy_clone = Rc::clone(&smuggled_rc); // Increment `strong_count` of `Rc`
                                                            // When `_dummy_clone` is dropped, `strong_count` is decremented.
            }
        });
        handles.push(child_handle);
    }
    for handle in handles {
        handle.join().expect("failed to join child thread");
    }

    assert_eq!(Rc::strong_count(arc_parent.read().get(&0).unwrap()), 1);
}

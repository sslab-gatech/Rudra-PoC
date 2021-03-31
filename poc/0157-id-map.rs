/*!
```rudra-poc
[target]
crate = "id-map"
version = "0.2.1"

[[target.peer]]
crate = "id-set"
version = "0.2.1"

[report]
issue_url = "https://github.com/andrewhickman/id-map/issues/3"
issue_date = 2021-02-26
rustsec_url = "https://github.com/RustSec/advisory-db/pull/863"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
bug_count = 3
rudra_report_locations = [
    "src/lib.rs:369:5: 381:6",
    "src/lib.rs:167:5: 184:6",
    "src/lib.rs:188:5: 207:6",
]
```
!*/
#![forbid(unsafe_code)]

use id_map::IdMap;
use id_set::IdSet;

struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}
impl Clone for DropDetector {
    fn clone(&self) -> Self { panic!("Panic on clone!"); }
}

fn main() {
    //clone_from_panic_will_drop_invalid_memory();
    //get_or_insert_with_leaves_state_inconsistent();
    remove_set_leaves_state_inconsistent_if_drop_panics();
}

fn clone_from_panic_will_drop_invalid_memory() {
    let mut map = IdMap::new();
    map.insert(DropDetector(1));

    let mut map_2 = IdMap::new();
    map_2.insert(DropDetector(2));
    map_2.clone_from(&map);
}

fn get_or_insert_with_leaves_state_inconsistent() {
    let mut map : IdMap<DropDetector> = IdMap::with_capacity(0);
    map.get_or_insert_with(0, || panic!("Panic in insertion function!"));
}

struct PanicsOnDrop(u32, bool);

impl Drop for PanicsOnDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);

        if (self.1) {
            self.1 = false;
            panic!("Panicking on drop");
        }
    }
}

fn remove_set_leaves_state_inconsistent_if_drop_panics() {
    let mut map = IdMap::new();
    map.insert(PanicsOnDrop(1, true));

    map.remove_set(&IdSet::new_filled(1));
}

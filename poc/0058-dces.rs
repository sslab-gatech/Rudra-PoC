/*!
```rudra-poc
[target]
crate = "dces"
version = "0.2.0"

[report]
issue_url = "https://gitlab.redox-os.org/redox-os/dces-rust/-/issues/8"
issue_date = 2020-12-09
rustsec_url = "https://github.com/RustSec/advisory-db/pull/590"
rustsec_id = "RUSTSEC-2020-0139"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
rudra_report_locations = ["src/world.rs:37:1: 42:2"]
sendsync_details = ["NeedSend"]
```
!*/
#![forbid(unsafe_code)]

use dces::entity::EntityStore;
use dces::prelude::*;

use std::rc::Rc;

struct MyEntityStore {
    entity_store_rc: Rc<i32>,
}

impl EntityStore for MyEntityStore {
    fn register_entity(&mut self, entity: impl Into<Entity>) {
        println!("Thread: {:p}", self.entity_store_rc);
        // Races with the Rc in the main thread.
        for _ in 0..100_000_000 {
            self.entity_store_rc.clone();
        }
    }

    fn remove_entity(&mut self, entity: impl Into<Entity>) {}
}

fn main() {
    let rc = Rc::new(42);

    let entity_store = MyEntityStore {
        entity_store_rc: rc.clone(),
    };
    let mut world = World::from_stores(entity_store, ComponentStore::default());

    std::thread::spawn(move || {
        world.create_entity().build();
    });

    println!("Main:   {:p}", rc);
    for _ in 0..100_000_000 {
        rc.clone();
    }
}

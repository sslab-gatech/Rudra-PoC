/*!
```rudra-poc
[target]
crate = "concread"
version = "0.2.6"
indexed_version = "0.1.18"

[report]
issue_url = "https://github.com/kanidm/concread/issues/55"
issue_date = 2021-02-25

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/collections/bptree/node.rs:376:5: 543:6"]
```
!*/
#![forbid(unsafe_code)]

use concread::bptree::BptreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DropDetector(u32);

impl Clone for DropDetector {
    fn clone(&self) -> Self {
        panic!("Panic on clone!");
    }
}

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let tree: BptreeMap<DropDetector, i32> = BptreeMap::new();
    let mut writer = tree.write();

    writer.insert(DropDetector(1), 1);
    writer.commit();

    let mut writer = tree.write();
    writer.insert(DropDetector(0), 0);
}

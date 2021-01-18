/*!
```rudra-poc
[target]
crate = "arc-swap"
version = "1.0.0"

[test]
analyzers = ["Manual", "SendSyncVariance"]
bug_classes = ["Other"]

[report]
issue_url = "https://github.com/vorner/arc-swap/issues/45"
issue_date = 2020-12-09
rustsec_url = "https://github.com/RustSec/advisory-db/pull/530"
rustsec_id = "RUSTSEC-2020-0091"
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

use arc_swap::access::Map;
use arc_swap::access::{Access, Constant};

static CORRECT_ADDR: &str = "I'm pointing to the correct location!";

#[derive(Clone)]
struct MemoryChecker {
    // should be always CORRECT_ADDR
    message: &'static str,
}

impl MemoryChecker {
    pub fn new() -> Self {
        MemoryChecker {
            message: CORRECT_ADDR,
        }
    }

    pub fn validate(&self) {
        println!(
            "Pointing to {:?}, len {}",
            self.message as *const _,
            self.message.len()
        );
        println!("Message: {}", self.message);
    }
}

fn overwrite() {
    let a = 123;
    let b = 456;
    println!("Overwriting stack content {} {}", a, b);
}

fn main() {
    let constant = Constant(MemoryChecker::new());
    constant.0.validate();

    // Create a map with identity mapping
    let map = Map::new(constant, |checker: &MemoryChecker| checker);

    // After calling this, `value` pointer of `MapGuard` points to a dangling stack address
    let loaded = map.load();

    overwrite();

    loaded.validate();
}

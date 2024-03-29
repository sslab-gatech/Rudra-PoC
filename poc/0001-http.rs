/*!
```rudra-poc
[target]
crate = "http"
version = "0.1.19"

[report]
issue_url = "hyperium/http#353 and hyperium/http#354"
issue_date = 2019-11-16
rustsec_url = "https://github.com/RustSec/advisory-db/pull/218"
rustsec_id = "RUSTSEC-2019-0034"

[[bugs]]
analyzer = "Manual"
bug_class = "Other"
bug_count = 2
rudra_report_locations = []
```
!*/
#![forbid(unsafe_code)]

use http::header::HeaderMap;

struct DropDetector(u32);

impl Drop for DropDetector {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    {
        println!("> Failing to drop Drain causes double-free");

        let mut map = HeaderMap::with_capacity(32);
        map.insert("1", DropDetector(1));
        map.insert("2", DropDetector(2));

        let mut drain = map.drain();
        drain.next();
        std::mem::forget(drain);
    }

    {
        println!("> Dropping drain without consuming it leaks memory");

        let mut map = HeaderMap::with_capacity(32);
        map.insert("3", DropDetector(3));
        map.insert("4", DropDetector(4));

        let mut drain = map.drain();
        drain.next();
    }

    {
        println!("> Data race in safe Rust");

        let mut map = HeaderMap::<u32>::with_capacity(8);
        map.insert("key1", 1);
        map.append("key1", 2);
        map.insert("key2", 3);
        map.append("key2", 4);

        let mut drain = map.drain();
        let (key1, mut val1) = drain.next().unwrap();
        let (key2, mut val2) = drain.next().unwrap();

        dbg!(val1.next());
        dbg!(val2.next());
        dbg!(val1.next());
        dbg!(val2.next());
    }
}

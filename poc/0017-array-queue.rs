/*!
```rudra-poc
[target]
crate = "array-queue"
version = "0.3.3"

[test]
analyzers = ["Manual"]

[report]
issue_url = "https://github.com/raviqqe/array-queue/issues/2"
issue_date = 2020-09-26
rustsec_url = "https://github.com/RustSec/advisory-db/pull/396"
rustsec_id = "RUSTSEC-2020-0047"
```
!*/
#![forbid(unsafe_code)]

use array_queue::ArrayQueue;

fn main() {
    {
        // 1. Allows reading of uninitialized memory.
        //
        // A queue of size 3 is setup like this:
        //     [x, x, x]    length = 0
        //      ^           start
        // where x is uninitialized memory.
        //
        // push_back(a); push_back(b); push_back(c)
        //     [a, b, c]    length = 3
        //      ^           start
        //
        // pop_front(); pop_back():
        //     [x, b, x]    length = 1
        //         ^        start
        //
        // At this point when performing a pop_back, the queue should use the
        // `ArrayQueue::index` method to index into the array properly but
        // instead simply uses `self.length - 1` causing it to read the first
        // x.
        // https://github.com/raviqqe/array-queue/blob/32fa10f8f15140fb64a4cf36a2a834f876c91056/src/array_queue.rs#L98
        let mut a: [u64; 32] = [0x41; 32];
        let mut x: ArrayQueue<[[u64; 32]; 3]> = ArrayQueue::new();
        x.push_back(&&a);
        x.push_back(&&a);
        x.push_back(&&a);

        x.pop_front().unwrap();
        x.pop_back().unwrap();

        let popped = x.pop_back().unwrap();
        println!("Contents of array: {:?}", popped);
        assert_eq!(popped[0], 0x41);
    }

    {
        // 2. Initializes memory with mem::uninitialized, this is instantly
        //    UB for types that cannot inhabit uninitialized. Should be
        //    changed over to MaybeUninit. (Triggers a panic on latest Rust).

        //let mut x: ArrayQueue<[Box<i32>; 3]> = ArrayQueue::new();
        //x.push_back(&Box::new(1));
    }
}

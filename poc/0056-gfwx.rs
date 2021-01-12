/*!
```rudra-poc
[target]
crate = "gfwx"
version = "0.3.0"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_date = 2020-12-08
issue_url = "https://github.com/Devolutions/gfwx-rs/issues/7"
```
!*/
#![forbid(unsafe_code)]

use gfwx::processing::image::Image;

use crossbeam_utils::thread;
use std::cell::Cell;

// A simple tagged union used to demonstrate problems with data races in Cell.
#[derive(Debug, Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let cell = Cell::new(RefOrInt::Ref(&SOME_INT));
    let slice = &mut [&cell];

    let image = Image::from_slice(slice, (1, 1), 1);

    let mut chunk_iterator = image.into_chunks_mut(1, 1);
    let image_chunk = chunk_iterator.next().unwrap();

    thread::scope(|s| {
        s.spawn(|_| {
            let smuggled_cell = image_chunk[(0, 0)];

            loop {
                // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                smuggled_cell.set(RefOrInt::Ref(&SOME_INT));
                smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
            }
        });

        loop {
            if let RefOrInt::Ref(addr) = cell.get() {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u64 == &SOME_INT as *const u64 {
                    continue;
                }

                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

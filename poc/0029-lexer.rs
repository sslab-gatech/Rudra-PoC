/*!
```rudra-poc
[target]
crate = "lexer"
version = "0.1.16"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[report]
issue_url = "https://gitlab.com/nathanfaucett/rs-lexer/-/issues/2"
issue_date = 2020-11-10
rustsec_url = "https://github.com/RustSec/advisory-db/pull/595"
rustsec_id = "RUSTSEC-2020-0138"

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
rudra_report_locations = ["src/reader_result.rs:15:1: 20:2"]
sendsync_details = ["NeedSync"]
```
!*/
#![forbid(unsafe_code)]

use lexer::ReaderResult;

use crossbeam_utils::thread;
use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
enum RefOrInt<'a> {
    Ref(&'a u64),
    Int(u64),
}
static SOME_INT: u64 = 123;

fn main() {
    let reader_result: ReaderResult<_, ()> =
        ReaderResult::Some(Cell::new(RefOrInt::Ref(&SOME_INT)));

    thread::scope(|s| {
        let reader_result_ref = &reader_result;
        s.spawn(move |_| {
            let sent_ref = reader_result_ref;
            if let ReaderResult::Some(smuggled_cell) = sent_ref {
                loop {
                    // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
                    smuggled_cell.set(RefOrInt::Ref(&SOME_INT));
                    smuggled_cell.set(RefOrInt::Int(0xdeadbeef));
                }
            }
        });

        if let ReaderResult::Some(cell) = reader_result_ref {
            loop {
                if let RefOrInt::Ref(addr) = cell.get() {
                    // Hope that between the time we pattern match the object as a
                    // `Ref`, it gets written to by the other thread.
                    if addr as *const u64 == &SOME_INT as *const u64 {
                        continue;
                    }

                    // Due to the data race, obtaining Ref(0xdeadbeef) is possible
                    println!("Pointer is now: {:p}", addr);
                    println!("Dereferencing addr will now segfault: {}", *addr);
                }
            }
        }
    });
}

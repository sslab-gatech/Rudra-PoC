/*!
```crux-poc
[target]
crate = "obstack"
version = "0.1.3"

[test]
analyzers = ["manual"]
cargo_flags = ["--release"]

[report]
title = "Obstack generates unaligned references"
description = """
Obstack generates unaligned references for types that require a large alignment."""
code_snippets = ["https://github.com/petertodd/rust-obstack/blob/e1dde0dbed709ebdea9bd1f79ec718e80c5d0bf6/src/lib.rs#L317-L337"]
patched = []
informational = "unsound"
issue_url = "https://github.com/petertodd/rust-obstack/issues/4"
issue_date = 2020-09-03
rustsec_url = "https://github.com/RustSec/advisory-db/pull/373"
```
!*/
#![forbid(unsafe_code)]

use obstack::Obstack;

#[repr(align(256))]
#[derive(Copy, Clone)]
struct LargeAlign(u8);

fn main() {
    // https://github.com/petertodd/rust-obstack/blob/e1dde0dbed709ebdea9bd1f79ec718e80c5d0bf6/src/lib.rs#L317-L337
    // Line 322: Incorrect padding bytes calculation. It should be `(alignment - (start_ptr % alignment)) % alignment`.
    // Line 329: Wasted memory due to `bytes_to_item()` not being applied to `padding`.

    // Due to the incorrect padding calculation, the code generates unaligned reference in release mode.
    let obstack = Obstack::new();
    let val_ref = obstack.push_copy(LargeAlign(0));

    let address = val_ref as *mut _ as usize;
    println!("{:x}", address);
    assert!(address % std::mem::align_of::<LargeAlign>() == 0);
}

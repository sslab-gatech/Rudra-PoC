/*!
```rudra-poc
[target]
crate = "obstack"
version = "0.1.3"

[test]
cargo_flags = ["--release"]

[report]
issue_url = "https://github.com/petertodd/rust-obstack/issues/4"
issue_date = 2020-09-03
rustsec_url = "https://github.com/RustSec/advisory-db/pull/373"
rustsec_id = "RUSTSEC-2020-0040"

[[bugs]]
analyzer = "Manual"
guide = "UnsafeDestructor"
bug_class = "Other"
bug_count = 2
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

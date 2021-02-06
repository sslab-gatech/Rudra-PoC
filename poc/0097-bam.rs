/*!
```rudra-poc
[target]
crate = "bam"
version = "0.1.2"

[test]
cargo_flags = ["--release"]

[report]
issue_url = "https://gitlab.com/tprodanov/bam/-/issues/4"
issue_date = 2021-01-07

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
rudra_report_locations = ["src/bgzip/mod.rs:296:5: 332:6"]

[[bugs]]
analyzer = "Manual"
guide = "UnsafeDataflow"
bug_class = "Other"
rudra_report_locations = []
```
!*/
#![forbid(unsafe_code)]

use bam::bgzip::Block;

pub const MAX_BLOCK_SIZE: usize = 65536;

const HEADER_SIZE: usize = 12;
const MIN_EXTRA_SIZE: usize = 6;
const FOOTER_SIZE: usize = 8;
// WRAPPER_SIZE = 26
const WRAPPER_SIZE: usize = HEADER_SIZE + MIN_EXTRA_SIZE + FOOTER_SIZE;

pub const MAX_COMPRESSED_SIZE: usize = MAX_BLOCK_SIZE - WRAPPER_SIZE;

fn main() {
    let mut block = Block::new();

    let mut buf: Vec<u8> = Vec::new();
    buf.extend(&[31, 139, 8, 4, 5, 6, 7, 8, 9, 10, 0, 0]); // 12 bytes header
    buf.extend(&[66, 67, 2, 0, 0, 0]); // `block_size` is 0

    block.load(None, &mut buf.as_slice()).ok();

    let compressed_size = block.compressed_size();
    println!("{}", compressed_size);
    assert!(compressed_size as usize <= MAX_COMPRESSED_SIZE + FOOTER_SIZE);
}

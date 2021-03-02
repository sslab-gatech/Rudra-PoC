/*!
```rudra-poc
[target]
crate = "byte_struct"
version = "0.6.0"

[report]
issue_url = "https://github.com/wwylele/byte-struct-rs/issues/1"
issue_date = 2021-03-01
rustsec_url = "https://github.com/RustSec/advisory-db/pull/796"
rustsec_id = "RUSTSEC-2021-0032"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/lib.rs:410:13: 422:14"]
```
!*/
#![forbid(unsafe_code)]

use byte_struct::*;

// Custom type that panics when reading bytes.
struct CustomByteStruct(u8);

impl ByteStructLen for CustomByteStruct {
    const BYTE_LEN: usize = 1;
}

impl ByteStruct for CustomByteStruct {
    fn write_bytes(&self, bytes: &mut [u8]) { }
    fn read_bytes(bytes: &[u8]) -> Self { panic!("Panic when reading") }
}

impl Drop for CustomByteStruct {
    fn drop(&mut self) { println!("Dropping {}", self.0) }
}

// Wrapper around the type above so we can use the
// `ByteStructUnspecifiedByteOrder for [T; $x]` impl.
#[derive(ByteStruct)]
#[byte_struct_le]
struct ArrayOfCustomByteStruct {
    custom_structs: [CustomByteStruct; 2]
}

fn main() {
    let bytes = [0x01, 0x02];
    let deserialized = ArrayOfCustomByteStruct::read_bytes(&bytes[..]);
}

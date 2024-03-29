```toml
[advisory]
id = "RUSTSEC-0000-0000"
package = "bronzedb-protocol"
date = "2021-01-03"
url = "()"
categories = ["memory-exposure"]

[versions]
patched = []
```

# reading on uninitialized buffer can cause UB (impl of `ReadKVExt`)

Affected versions of this crate passes an uninitialized buffer to a user-provided `Read` implementation.

Arbitrary `Read` implementations can read from the uninitialized buffer (memory exposure) and also can return incorrect number of bytes written to the buffer.
Reading from uninitialized memory produces undefined values that can quickly invoke undefined behavior.
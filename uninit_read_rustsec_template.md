# ((Issue Title))

Original issue report: ((Link to issue))

As of Jan 2021, there doesn't seem to be an ideal fix that works in stable Rust with no performance overhead. Below are links to relevant discussions & suggestions for the fix.

* [Well-written document regarding the issue](https://paper.dropbox.com/doc/IO-Buffer-Initialization-MvytTgjIOTNpJAS6Mvw38)
* [Rust RFC 2930](https://github.com/rust-lang/rfcs/blob/master/text/2930-read-buf.md#summary)
* [nightly feature `std::io::Initializer`](https://doc.rust-lang.org/std/io/struct.Initializer.html)
* [Discussion in Rust Internals Forum](https://internals.rust-lang.org/t/uninitialized-memory/1652)

Thank you for reviewing this PR :)
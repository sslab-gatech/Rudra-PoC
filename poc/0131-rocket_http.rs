/*!
```rudra-poc
[target]
crate = "rocket_http"
version = "0.4.6"
indexed_version = "0.4.5"

[test]
cargo_toolchain = "nightly"

[report]
issue_url = "https://github.com/SergioBenitez/Rocket/issues/1534"
issue_date = 2021-02-09

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "PanicSafety"
rudra_report_locations = ["src/uri/formatter.rs:334:5: 357:6"]
```
!*/
#![forbid(unsafe_code)]

use rocket_http::uri::{Formatter, Query, UriDisplay};

struct MyDisplay;

struct MyValue;

impl UriDisplay<Query> for MyValue {
    fn fmt(&self, _f: &mut Formatter<Query>) -> std::fmt::Result {
        panic!()
    }
}

struct Wrapper<'a, 'b>(&'a mut Formatter<'b, Query>);

impl UriDisplay<Query> for MyDisplay {
    fn fmt(&self, formatter: &mut Formatter<Query>) -> std::fmt::Result {
        let wrapper = Wrapper(formatter);
        let temporary_string = String::from("hello");
        wrapper.0.write_named_value(&temporary_string, MyValue)
    }
}

impl<'a, 'b> Drop for Wrapper<'a, 'b> {
    fn drop(&mut self) {
        let _overlap = String::from("12345");
        self.0.write_raw("world").ok();
    }
}

fn main() {
    println!("{}", &MyDisplay as &dyn UriDisplay<Query>);
}

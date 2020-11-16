/*!
```crux-poc
[target]
crate = "generator"
version = "0.6.23"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "Generator<_, _, T> lacks a `T: Send` bound on its Send trait allowing data races"
description = """
issue description"""
code_snippets = ["https://github.com/Xudong-Huang/generator-rs/blob/4af7740e18fb2be4ef2adbf0e4dd7b1889e0099b/src/gen_impl.rs#L27"]
patched = []
informational = "unsound"
issue_url = "https://github.com/Xudong-Huang/generator-rs/issues/27"
issue_date = 2020-11-16
```
!*/
#![forbid(unsafe_code)]

use generator::Gn;
use std::{rc::Rc, thread};

fn main() {
    let rc = Rc::new(());

    let rc_clone = rc.clone();
    let mut generator = Gn::new_scoped(move |_| {
        return rc_clone;
    });

    let child = thread::spawn(move || {
        let smuggled_rc = generator.next().unwrap();

        println!("RC in thread: {:p}", smuggled_rc);
        for _ in 0..1000000000 {
            let x = smuggled_rc.clone();
        }
    });

    println!("RC in main: {:p}", rc);
    for _ in 0..1000000000 {
        let x = rc.clone();
    }

    child.join().unwrap();
    assert_eq!(Rc::strong_count(&rc), 2);
}

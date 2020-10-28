/*!
```crux-poc
[target]
crate = "beef"
version = "0.4.4"

[[target.peer]]
crate = "crossbeam-utils"
version = "0.8.0"

[test]
analyzers = ["SendSyncChecker"]

[report]
title = "beef::Cow lacks a Sync bound on its Send trait allowing for data races"
description = """
`beef::Cow` implements `Send` for any type that implements `Send`. However,
it allows multiple read-only references to the underlying data creating a read
data race. This allows objects like `RefCell` that implement `Send` but not
`Sync` to be shared across threads leading to undefined behavior.
"""
code_snippets = ["https://github.com/maciejhirsz/beef/blob/0b4685143e680749991c295836d8d09565fd6814/src/generic.rs#L531"]
patched = []
informational = "unsound"
```
!*/
#![forbid(unsafe_code)]

use crossbeam_utils::thread;
use std::cell::Cell;
use std::rc::Rc;

use std::cell::RefCell;
use std::sync::mpsc;
use std::thread::sleep;
use std::time;

use beef::Cow;

fn main() {
    let x = [Cell::new(42)];

    // A simple proof-of-concept demonstrating how a loose bound on Cow's
    // Send trait allows non-`Sync` but `Send`able objects to be shared across
    // threads.
    thread::scope(|s| {
        let cow1: Cow<[Cell<i32>]> = Cow::borrowed(&x[..]);
        let cow2: Cow<[Cell<i32>]> = cow1.clone();

        let child = s.spawn(|_| {
            let smuggled = cow2.unwrap_borrowed();
            smuggled[0].set(2);

            // Print the value of the Cow-value and the address of the
            // underlying Cell.
            println!("{:?}, {:p}", smuggled, &smuggled[0]);
        });
        child.join().unwrap();

        // This should print the same address as above indicating the underlying
        // `Cell` in x is now shared across threads, a violation of `!Sync`.
        println!("{:?}, {:p}", x, &x[0]);
    });

    // A simple tagged union used to demonstrate the problems with data races
    // in RefCells. Since RefCell has no synchronization methods, it is possible
    // for two threads to end up breaking the usual borrowing rules.
    //
    // For example, given the right timing, RefCell::borrow_mut can succeed on
    // both threads which then end up holding mutable references to the same
    // underlying object.
    //
    // In this particular example, we show how a shared RefCell can lead to a
    // null pointer derference. Our main thread pattern matches on the `Ref`
    // version of the enum while the other thread changes the underlying memory
    // to an `Int(0)`. Interpreted as a `Ref` this causes a null pointer to be
    // dereferenced.
    #[derive(Debug, Clone)]
    enum RefOrInt<'a> {
        Ref(&'a u64),
        Int(u64)
    }

    let some_int : u64 = 42;
    let y = [RefCell::new(RefOrInt::Ref(&some_int))];

    thread::scope(|s| {
        // Set up channels so the threads can communicate whether they managed
        // to data-race successfully.
        let (tx_thread_result, rx_thread_result) = mpsc::channel();
        let (tx_main_result, rx_main_result) = mpsc::channel();

        let cow1: Cow<[RefCell<RefOrInt>]> = Cow::borrowed(&y[..]);
        let cow2: Cow<[RefCell<RefOrInt>]> = cow1.clone();

        let child = s.spawn(move |_| {
            let smuggled = cow2.unwrap_borrowed();

            loop {
                // Try to borrow the RefCell mutably.
                let borrow_result = smuggled[0].try_borrow_mut();
                // Send over our result, whether our borrow was successful or not.
                tx_thread_result.send(borrow_result.is_ok()).unwrap();
                // Get their result, whether their borrow was successful or not.
                let main_thread_result = rx_main_result.recv().unwrap();

                // Cool, we both managed to borrow successfully.
                if (main_thread_result && borrow_result.is_ok()) {
                    println!("Borrowed mutably! - Thread");

                    // Allow the other thread some time to pattern-match on
                    // the enum and extract it as a Ref.
                    sleep(time::Duration::from_millis(10));

                    // Now change over the enum to an Int(0).
                    *borrow_result.unwrap() = RefOrInt::Int(0);
                }
            }
        });

        loop {
            // Try to borrow the RefCell mutably.
            let borrow_result = y[0].try_borrow_mut();
            // Send over our result, whether our borrow was successful or not.
            tx_main_result.send(borrow_result.is_ok()).unwrap();
            // Get their result, whether their borrow was successful or not.
            let other_thread_result = rx_thread_result.recv().unwrap();

            // Cool, we both managed to borrow successfully.
            if (other_thread_result && borrow_result.is_ok()) {
                println!("Borrowed mutably! - Main");

                // Pattern match on the enum to pull out the reference.
                if let RefOrInt::Ref(ref mut internal_ref) = *borrow_result.unwrap() {
                    println!("Initial destructure: {}", internal_ref);

                    // Allow the other thread to change the pointed-to enum to
                    // a null pointer.
                    sleep(time::Duration::from_millis(50));

                    // We still hold a &u64 here as part of `internal_ref` but
                    // the underlying memory has been changed to 0 at this
                    // point, the print will now cause a null pointer deref.
                    println!("About to dereference ptr again");
                    println!("Second dereference: {}", internal_ref);
                }
            }
        }
    });
}
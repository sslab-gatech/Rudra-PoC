/*!
```rudra-poc
[target]
crate = "disrustor"
version = "0.2.0"

[report]
issue_url = "https://github.com/sklose/disrustor/issues/1"
issue_date = 2020-12-17

[[bugs]]
analyzer = "SendSyncVariance"
bug_class = "SendSyncVariance"
bug_count = 2
rudra_report_locations = ["src/ringbuffer.rs:48:1: 48:41", "src/ringbuffer.rs:49:1: 49:41"]

[[bugs]]
analyzer = "Manual"
guide = "SendSyncVariance"
bug_class = "Other"
rudra_report_locations = []
```
!*/
#![forbid(unsafe_code)]

use std::cell::Cell;
use std::sync::Arc;
use std::thread;

use disrustor::internal::RingBuffer;
use disrustor::DisrustorBuilder;
use disrustor::EventProducer;

// A simple tagged union used to demonstrate problems with data races in Cell.
#[derive(Clone, Copy)]
enum RefOrInt {
    Ref(&'static u64),
    Int(u64),
}

static STATIC_INT: u64 = 123;

impl Default for RefOrInt {
    fn default() -> Self {
        RefOrInt::Ref(&STATIC_INT)
    }
}

fn main() {
    let provider = Arc::new(RingBuffer::<Cell<RefOrInt>>::new(1));
    let provider_cloned = provider.clone();

    thread::spawn(move || {
        let (_executor, producer) = DisrustorBuilder::new(provider_cloned)
            .with_spin_wait()
            .with_single_producer()
            .with_barrier(|_| {})
            .build();

        producer.write(std::iter::once(()), |slot, _seq, _item| loop {
            // Repeatedly write Ref(&addr) and Int(0xdeadbeef) into the cell.
            *slot.get_mut() = RefOrInt::Ref(&STATIC_INT);
            *slot.get_mut() = RefOrInt::Int(0xdeadbeef);
        });
    });

    let (_executor, producer) = DisrustorBuilder::new(provider.clone())
        .with_spin_wait()
        .with_single_producer()
        .with_barrier(|_| {})
        .build();

    producer.write(std::iter::once(()), |slot, _seq, _item| {
        loop {
            if let RefOrInt::Ref(addr) = slot.get() {
                // Hope that between the time we pattern match the object as a
                // `Ref`, it gets written to by the other thread.
                if addr as *const u64 == &STATIC_INT as *const u64 {
                    continue;
                }

                println!("Pointer is now: {:p}", addr);
                println!("Dereferencing addr will now segfault: {}", *addr);
            }
        }
    });
}

/*!
```rudra-poc
[target]
crate = "eventio"
version = "0.4.0"

[[target.peer]]
crate = "crossbeam-channel"
version = "0.5.0"

[[target.peer]]
crate = "pcap-parser"
version = "0.9.3"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/petabi/eventio/issues/33"
issue_date = 2020-12-20
rustsec_url = "https://github.com/RustSec/advisory-db/pull/600"
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]

use crossbeam_channel::unbounded;
use eventio::{pcap, Input};
use pcap_parser::{LegacyPcapBlock, PcapHeader, ToVec};
use std::{
    cell::Cell,
    io::{self, Cursor},
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

/// Non-Send object implementing `Read` trait.
struct CustomRead {
    read: Cursor<Vec<u8>>,
    non_send_counter: Rc<Cell<usize>>,
    atomic_cnt: bool,
}
impl CustomRead {
    fn new(non_send_counter: Rc<Cell<usize>>, atomic_cnt: bool) -> Self {
        CustomRead {
            read: Self::create_pcap(),
            non_send_counter,
            atomic_cnt,
        }
    }
    fn create_pcap() -> Cursor<Vec<u8>> {
        let fake_content = b"fake packet";
        let pkt = LegacyPcapBlock {
            ts_sec: 0,
            ts_usec: 0,
            caplen: fake_content.len() as u32,
            origlen: fake_content.len() as u32,
            data: fake_content,
        }
        .to_vec_raw()
        .unwrap();
        let mut buf = PcapHeader::new().to_vec_raw().unwrap();
        for _ in 0..N_PACKETS {
            buf.extend(pkt.iter());
        }
        Cursor::new(buf)
    }
    fn update_read_cnt(&self) {
        if self.atomic_cnt {
            REAL_CNT.fetch_add(1, Ordering::Release);
        } else {
            self.non_send_counter.set(self.non_send_counter.get() + 1);
        }
    }
}
impl std::io::Read for CustomRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Increment counter to keep track of '# of read events'.
        self.update_read_cnt();
        self.read.read(buf)
    }
}

const N_PACKETS: usize = 9_000_000;
const NTHREADS: usize = 8;
static REAL_CNT: AtomicUsize = AtomicUsize::new(0);
fn experiment(atomic_cnt: bool) -> usize {
    let (data_tx, data_rx) = unbounded();
    let (ack_tx, ack_rx) = unbounded();

    // Non-Sync counter that counts the # of read events that happen within this thread.
    // This counter should only be updated within a single thread.
    // Updating this counter concurrently from multiple threads will result in incorrect counts.
    let read_cnt_in_thread = Rc::new(Cell::new(0_usize));

    let mut children = Vec::with_capacity(NTHREADS);
    for _ in 0..NTHREADS {
        let input = pcap::Input::with_read(
            data_tx.clone(),
            ack_rx.clone(),
            CustomRead::new(Rc::clone(&read_cnt_in_thread), atomic_cnt),
        );
        children.push(thread::spawn(move || {
            // `input` is moved from parent thread to child thread.
            input.run().unwrap()
        }));
    }

    std::mem::drop(data_tx);
    for ev in data_rx.iter() {
        ack_tx.send(ev.seq_no).unwrap();
    }
    std::mem::drop(ack_tx);

    for child in children.into_iter() {
        child.join().unwrap();
    }
    if atomic_cnt {
        REAL_CNT.load(Ordering::Acquire)
    } else {
        read_cnt_in_thread.get()
    }
}

fn main() {
    // Check that `read_cnt_in_thread` maintains incorrect count of events.
    assert_eq!(experiment(true), experiment(false),);
}

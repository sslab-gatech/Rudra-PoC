/*!
```rudra-poc
[target]
crate = "va-ts"
version = "0.0.3"

[test]
analyzers = ["SendSyncVariance"]
bug_classes = ["SendSyncVariance"]

[report]
issue_url = "https://github.com/video-audio/va-ts/issues/4"
issue_date = 2020-12-22
rustsec_url = "https://github.com/RustSec/advisory-db/pull/642"
unique_bugs = 1
```
!*/
#![forbid(unsafe_code)]
use std::ops::Drop;
use std::sync::{Mutex, MutexGuard};
use std::thread::{self, ThreadId};
use va_ts::{DemuxedPacket, DemuxedTable, Demuxer, DemuxerEvents, SubtableID};

struct X(MutexGuard<'static, u64>, ThreadId);
impl DemuxerEvents for X {
    fn on_table(&mut self, _: SubtableID, _: &DemuxedTable) {}
    fn on_packet(&mut self, _: &DemuxedPacket) {}
}
impl Drop for X {
    fn drop(&mut self) {
        // `MutexGuard` must not be dropped from a thread that didn't lock the `Mutex`.
        //
        // If a thread attempts to unlock a Mutex that it has not locked, it can result in undefined behavior.
        // (https://github.com/rust-lang/rust/issues/23465#issuecomment-82730326)
        assert_eq!(self.1, thread::current().id());
    }
}

fn main() {
    let static_mutex = Box::leak(Box::new(Mutex::new(0xbeefbeef_u64)));
    // MutexGuard is `!Send`
    let mutex_guard = static_mutex.lock().unwrap();
    let tid = thread::current().id();

    let demuxer = Demuxer::new(X(mutex_guard, tid));
    std::thread::spawn(move || {
        let demuxer = demuxer;

        // demuxer is dropped here, along with `MutexGuard`.
    })
    .join()
    .unwrap();
}

# Rudra-PoC

This repository contains the list of memory safety and soundness bugs found during the Rudra project.

See [DEV.md](./DEV.md) for the technical detail.

| ID | Crate | Method | Issue URL | RustSec ID |
| -- | ----- | ------ | --------- | ---------- |
| 0000 | rulinalg | <ul><li>manual</li></ul> | https://github.com/AtheMathmo/rulinalg/issues/201 | RUSTSEC-2020-0023 |
| 0001 | http | <ul><li>manual</li></ul> | N/A | RUSTSEC-2019-0034 |
| 0002 | http | <ul><li>manual</li></ul> | https://github.com/hyperium/http/issues/352 | RUSTSEC-2019-0033 |
| 0003 | ozone | <ul><li>manual</li></ul> | N/A | RUSTSEC-2020-0022 |
| 0004 | rocket | <ul><li>manual</li></ul> | https://github.com/SergioBenitez/Rocket/issues/1312 | RUSTSEC-2020-0028 |
| 0005 | failure | <ul><li>manual</li></ul> | https://github.com/rust-lang-nursery/failure/issues/336 | RUSTSEC-2019-0036 |
| 0006 | alpm-rs | <ul><li>UnsafeDestructor</li></ul> | https://github.com/pigeonhands/rust-arch/issues/2 | RUSTSEC-2020-0032 |
| 0007 | alg_ds | <ul><li>UnsafeDestructor</li></ul> | https://gitlab.com/dvshapkin/alg-ds/-/issues/1 | RUSTSEC-2020-0033 |
| 0008 | arr | <ul><li>manual</li><li>UnsafeDestructor</li><li>SendSyncChecker</li></ul> | https://github.com/sjep/array/issues/1 | RUSTSEC-2020-0034 |
| 0009 | chunky | <ul><li>manual</li><li>UnsafeDestructor</li></ul> | https://github.com/aeplay/chunky/issues/2 | RUSTSEC-2020-0035 |
| 0010 | crayon | <ul><li>manual</li><li>UnsafeDestructor</li></ul> | https://github.com/shawnscode/crayon/issues/87 | RUSTSEC-2020-0037 |
| 0011 | obstack | <ul><li>manual</li></ul> | https://github.com/petertodd/rust-obstack/issues/4 | RUSTSEC-2020-0040 |
| 0012 | ordnung | <ul><li>manual</li><li>UnsafeDestructor</li></ul> | https://github.com/maciejhirsz/ordnung/issues/8 | RUSTSEC-2020-0038 |
| 0013 | simple-slab | <ul><li>UnsafeDestructor</li></ul> | https://github.com/nathansizemore/simple-slab/issues/2 | RUSTSEC-2020-0039 |
| 0014 | sized-chunks | <ul><li>manual</li></ul> | https://github.com/bodil/sized-chunks/issues/11 | RUSTSEC-2020-0041 |
| 0015 | atom | <ul><li>SendSyncChecker</li></ul> | https://github.com/slide-rs/atom/issues/13 | RUSTSEC-2020-0044 |
| 0016 | stack | <ul><li>UnsafeDestructor</li></ul> | https://github.com/arcnmx/stack-rs/issues/4 | RUSTSEC-2020-0042 |
| 0017 | array-queue | <ul><li>manual</li></ul> | https://github.com/raviqqe/array-queue/issues/2 | RUSTSEC-2020-0047 |
| 0018 | dync | <ul><li>manual</li></ul> | https://github.com/elrnv/dync/issues/4 | RUSTSEC-2020-0050 |
| 0019 | futures | <ul><li>SendSyncChecker</li></ul> | https://github.com/rust-lang/futures-rs/issues/2239 | N/A |
| 0020 | beef | <ul><li>SendSyncChecker</li></ul> | https://github.com/maciejhirsz/beef/issues/37 | N/A |
| 0021 | futures-intrusive | <ul><li>SendSyncChecker</li></ul> | https://github.com/Matthias247/futures-intrusive/issues/53 | N/A |
| 0022 | atomic-option | <ul><li>SendSyncChecker</li></ul> | https://github.com/reem/rust-atomic-option/issues/4 | N/A |
| 0023 | convec | <ul><li>SendSyncChecker</li></ul> | https://github.com/krl/convec/issues/2 | N/A |
| 0024 | lock_api | <ul><li>SendSyncChecker</li></ul> | https://github.com/Amanieu/parking_lot/issues/258 | N/A |
| 0025 | im | <ul><li>SendSyncChecker</li></ul> | https://github.com/bodil/im-rs/issues/157 | N/A |
| 0026 | may_queue | <ul><li>SendSyncChecker</li></ul> | https://github.com/Xudong-Huang/may/issues/88 | N/A |
| 0027 | libsbc | <ul><li>SendSyncChecker</li></ul> | https://github.com/mvertescher/libsbc-rs/issues/4 | N/A |
| 0028 | lever | <ul><li>SendSyncChecker</li></ul> | https://github.com/vertexclique/lever/issues/15 | N/A |
| 0029 | lexer | <ul><li>SendSyncChecker</li></ul> | https://gitlab.com/nathanfaucett/rs-lexer/-/issues/2 | N/A |
| 0030 | cache | <ul><li>SendSyncChecker</li></ul> | https://github.com/krl/cache/issues/1 | N/A |
| 0031 | abox | <ul><li>SendSyncChecker</li></ul> | https://github.com/SonicFrog/abox/issues/1 | N/A |
| 0032 | conqueue | <ul><li>SendSyncChecker</li></ul> | https://github.com/longshorej/conqueue/issues/9 | N/A |
| 0033 | hashconsing | <ul><li>SendSyncChecker</li></ul> | https://github.com/AdrienChampion/hashconsing/issues/1 | N/A |
| 0034 | model | <ul><li>SendSyncChecker</li></ul> | https://github.com/spacejam/model/issues/3 | N/A |
| 0035 | late-static | <ul><li>SendSyncChecker</li></ul> | https://github.com/Richard-W/late-static/issues/1 | N/A |
| 0036 | bunch | <ul><li>SendSyncChecker</li></ul> | https://github.com/krl/bunch/issues/1 | N/A |
| 0037 | concread | <ul><li>SendSyncChecker</li></ul> | https://github.com/kanidm/concread/issues/48 | N/A |
| 0038 | parc | <ul><li>SendSyncChecker</li></ul> | https://github.com/hyyking/rustracts/pull/6 | N/A |
| 0039 | rcu_cell | <ul><li>SendSyncChecker</li></ul> | https://github.com/Xudong-Huang/rcu_cell/issues/3 | N/A |
| 0040 | appendix | <ul><li>SendSyncChecker</li></ul> | https://github.com/krl/appendix/issues/6 | N/A |
| 0041 | unicycle | <ul><li>SendSyncChecker</li></ul> | https://github.com/udoprog/unicycle/issues/8 | N/A |
| 0042 | toolshed | <ul><li>SendSyncChecker</li></ul> | https://github.com/ratel-rust/toolshed/issues/12 | N/A |
| 0043 | scottqueue | <ul><li>SendSyncChecker</li></ul> | https://github.com/rossdylan/rust-scottqueue/issues/1 | N/A |
| 0044 | signal-simple | <ul><li>SendSyncChecker</li></ul> | https://github.com/kitsuneninetails/signal-rust/issues/2 | N/A |
| 0045 | ruspiro-singleton | <ul><li>SendSyncChecker</li></ul> | https://github.com/RusPiRo/ruspiro-singleton/issues/10 | N/A |
| 0046 | generator | <ul><li>SendSyncChecker</li></ul> | https://github.com/Xudong-Huang/generator-rs/issues/27 | N/A |
| 0047 | try-mutex | <ul><li>SendSyncChecker</li></ul> | https://github.com/mpdn/try-mutex/issues/2 | N/A |
| 0048 | ticketed_lock | <ul><li>SendSyncChecker</li></ul> | https://github.com/kvark/ticketed_lock/issues/7 | N/A |
| 0049 | slock | <ul><li>SendSyncChecker</li></ul> | https://github.com/BrokenLamp/slock-rs/issues/2 | N/A |
| 0050 | magnetic | <ul><li>SendSyncChecker</li></ul> | https://github.com/johnshaw/magnetic/issues/9 | N/A |
| 0051 | syncpool | <ul><li>SendSyncChecker</li></ul> | https://github.com/Chopinsky/byte_buffer/issues/2 | N/A |
| 0052 | reffers | <ul><li>SendSyncChecker</li></ul> | https://github.com/diwic/reffers-rs/issues/7 | N/A |

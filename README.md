# Rudra-PoC

This repository contains the list of memory safety and soundness bugs found during the Rudra project.

Contributors: See [REPORTING.md](./REPORTING.md) for the reporting guideline.

| ID | Crate | Method | Issue Report | RustSec ID |
| -- | ----- | ------ | ------------ | ---------- |
| [0000](poc/0000-rulinalg.rs) | [rulinalg](https://crates.io/crates/rulinalg) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/AtheMathmo/rulinalg/201?label=AtheMathmo%2frulinalg%23201&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/AtheMathmo/rulinalg/issues/201) | [![RUSTSEC-2020-0023](https://img.shields.io/badge/RUSTSEC-2020--0023-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0023.html) |
| [0001](poc/0001-http.rs) | [http](https://crates.io/crates/http) | - manual | hyperium/http#353 and hyperium/http#354 | [![RUSTSEC-2019-0034](https://img.shields.io/badge/RUSTSEC-2019--0034-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2019-0034.html) |
| [0002](poc/0002-http.rs) | [http](https://crates.io/crates/http) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/hyperium/http/352?label=hyperium%2fhttp%23352&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/hyperium/http/issues/352) | [![RUSTSEC-2019-0033](https://img.shields.io/badge/RUSTSEC-2019--0033-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2019-0033.html) |
| [0003](poc/0003-ozone.rs) | [ozone](https://crates.io/crates/ozone) | - manual | N/A | [![RUSTSEC-2020-0022](https://img.shields.io/badge/RUSTSEC-2020--0022-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0022.html) |
| [0004](poc/0004-rocket.rs) | [rocket](https://crates.io/crates/rocket) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/SergioBenitez/Rocket/1312?label=SergioBenitez%2fRocket%231312&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/SergioBenitez/Rocket/issues/1312) | [![RUSTSEC-2020-0028](https://img.shields.io/badge/RUSTSEC-2020--0028-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0028.html) |
| [0005](poc/0005-failure.rs) | [failure](https://crates.io/crates/failure) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rust-lang-nursery/failure/336?label=rust-lang-nursery%2ffailure%23336&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rust-lang-nursery/failure/issues/336) | [![RUSTSEC-2019-0036](https://img.shields.io/badge/RUSTSEC-2019--0036-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2019-0036.html) |
| [0006](poc/0006-alpm-rs.rs) | [alpm-rs](https://crates.io/crates/alpm-rs) | - UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/pigeonhands/rust-arch/2?label=pigeonhands%2frust-arch%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/pigeonhands/rust-arch/issues/2) | [![RUSTSEC-2020-0032](https://img.shields.io/badge/RUSTSEC-2020--0032-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0032.html) |
| [0007](poc/0007-alg_ds.rs) | [alg_ds](https://crates.io/crates/alg_ds) | - UnsafeDestructor<br>- PanicSafety | [![GitLab issue](https://img.shields.io/badge/dvshapkin%2falg--ds%231-grey?logo=GitLab&style=flat-square)](https://gitlab.com/dvshapkin/alg-ds/-/issues/1) | [![RUSTSEC-2020-0033](https://img.shields.io/badge/RUSTSEC-2020--0033-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0033.html) |
| [0008](poc/0008-arr.rs) | [arr](https://crates.io/crates/arr) | - manual<br>- UnsafeDestructor<br>- SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/sjep/array/1?label=sjep%2farray%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/sjep/array/issues/1) | [![RUSTSEC-2020-0034](https://img.shields.io/badge/RUSTSEC-2020--0034-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0034.html) |
| [0009](poc/0009-chunky.rs) | [chunky](https://crates.io/crates/chunky) | - manual<br>- UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/aeplay/chunky/2?label=aeplay%2fchunky%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/aeplay/chunky/issues/2) | [![RUSTSEC-2020-0035](https://img.shields.io/badge/RUSTSEC-2020--0035-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0035.html) |
| [0010](poc/0010-crayon.rs) | [crayon](https://crates.io/crates/crayon) | - manual<br>- UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/shawnscode/crayon/87?label=shawnscode%2fcrayon%2387&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/shawnscode/crayon/issues/87) | [![RUSTSEC-2020-0037](https://img.shields.io/badge/RUSTSEC-2020--0037-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0037.html) |
| [0011](poc/0011-obstack.rs) | [obstack](https://crates.io/crates/obstack) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/petertodd/rust-obstack/4?label=petertodd%2frust-obstack%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/petertodd/rust-obstack/issues/4) | [![RUSTSEC-2020-0040](https://img.shields.io/badge/RUSTSEC-2020--0040-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0040.html) |
| [0012](poc/0012-ordnung.rs) | [ordnung](https://crates.io/crates/ordnung) | - manual<br>- UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/maciejhirsz/ordnung/8?label=maciejhirsz%2fordnung%238&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/maciejhirsz/ordnung/issues/8) | [![RUSTSEC-2020-0038](https://img.shields.io/badge/RUSTSEC-2020--0038-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0038.html) |
| [0013](poc/0013-simple-slab.rs) | [simple-slab](https://crates.io/crates/simple-slab) | - UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/nathansizemore/simple-slab/2?label=nathansizemore%2fsimple-slab%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/nathansizemore/simple-slab/issues/2) | [![RUSTSEC-2020-0039](https://img.shields.io/badge/RUSTSEC-2020--0039-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0039.html) |
| [0014](poc/0014-sized-chunks.rs) | [sized-chunks](https://crates.io/crates/sized-chunks) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/bodil/sized-chunks/11?label=bodil%2fsized-chunks%2311&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/bodil/sized-chunks/issues/11) | [![RUSTSEC-2020-0041](https://img.shields.io/badge/RUSTSEC-2020--0041-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0041.html) |
| [0015](poc/0015-atom.rs) | [atom](https://crates.io/crates/atom) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/slide-rs/atom/13?label=slide-rs%2fatom%2313&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/slide-rs/atom/issues/13) | [![RUSTSEC-2020-0044](https://img.shields.io/badge/RUSTSEC-2020--0044-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0044.html) |
| [0016](poc/0016-stack.rs) | [stack](https://crates.io/crates/stack) | - UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/arcnmx/stack-rs/4?label=arcnmx%2fstack-rs%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/arcnmx/stack-rs/issues/4) | [![RUSTSEC-2020-0042](https://img.shields.io/badge/RUSTSEC-2020--0042-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0042.html) |
| [0017](poc/0017-array-queue.rs) | [array-queue](https://crates.io/crates/array-queue) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/raviqqe/array-queue/2?label=raviqqe%2farray-queue%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/raviqqe/array-queue/issues/2) | [![RUSTSEC-2020-0047](https://img.shields.io/badge/RUSTSEC-2020--0047-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0047.html) |
| [0018](poc/0018-dync.rs) | [dync](https://crates.io/crates/dync) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/elrnv/dync/4?label=elrnv%2fdync%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/elrnv/dync/issues/4) | [![RUSTSEC-2020-0050](https://img.shields.io/badge/RUSTSEC-2020--0050-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0050.html) |
| [0019](poc/0019-futures.rs) | [futures](https://crates.io/crates/futures) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rust-lang/futures-rs/2239?label=rust-lang%2ffutures-rs%232239&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rust-lang/futures-rs/issues/2239) | [![RUSTSEC-2020-0059](https://img.shields.io/badge/RUSTSEC-2020--0059-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0059.html) |
| [0020](poc/0020-beef.rs) | [beef](https://crates.io/crates/beef) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/maciejhirsz/beef/37?label=maciejhirsz%2fbeef%2337&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/maciejhirsz/beef/issues/37) | Not Reported Yet |
| [0021](poc/0021-futures-intrusive.rs) | [futures-intrusive](https://crates.io/crates/futures-intrusive) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Matthias247/futures-intrusive/53?label=Matthias247%2ffutures-intrusive%2353&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Matthias247/futures-intrusive/issues/53) | [![RUSTSEC-2020-0072](https://img.shields.io/badge/RUSTSEC-2020--0072-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0072.html) |
| [0022](poc/0022-atomic-option.rs) | [atomic-option](https://crates.io/crates/atomic-option) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/reem/rust-atomic-option/4?label=reem%2frust-atomic-option%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/reem/rust-atomic-option/issues/4) | Not Reported Yet |
| [0023](poc/0023-convec.rs) | [convec](https://crates.io/crates/convec) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/krl/convec/2?label=krl%2fconvec%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/krl/convec/issues/2) | Not Reported Yet |
| [0024](poc/0024-lock_api.rs) | [lock_api](https://crates.io/crates/lock_api) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Amanieu/parking_lot/258?label=Amanieu%2fparking_lot%23258&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Amanieu/parking_lot/issues/258) | [![RUSTSEC-2020-0070](https://img.shields.io/badge/RUSTSEC-2020--0070-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0070.html) |
| [0025](poc/0025-im.rs) | [im](https://crates.io/crates/im) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/bodil/im-rs/157?label=bodil%2fim-rs%23157&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/bodil/im-rs/issues/157) | Not Reported Yet |
| [0026](poc/0026-may_queue.rs) | [may_queue](https://crates.io/crates/may_queue) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Xudong-Huang/may/88?label=Xudong-Huang%2fmay%2388&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Xudong-Huang/may/issues/88) | Not Reported Yet |
| [0027](poc/0027-libsbc.rs) | [libsbc](https://crates.io/crates/libsbc) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/mvertescher/libsbc-rs/4?label=mvertescher%2flibsbc-rs%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/mvertescher/libsbc-rs/issues/4) | Not Reported Yet |
| [0028](poc/0028-lever.rs) | [lever](https://crates.io/crates/lever) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/vertexclique/lever/15?label=vertexclique%2flever%2315&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/vertexclique/lever/issues/15) | Not Reported Yet |
| [0029](poc/0029-lexer.rs) | [lexer](https://crates.io/crates/lexer) | - SendSyncChecker | [![GitLab issue](https://img.shields.io/badge/nathanfaucett%2frs--lexer%232-grey?logo=GitLab&style=flat-square)](https://gitlab.com/nathanfaucett/rs-lexer/-/issues/2) | Not Reported Yet |
| [0030](poc/0030-cache.rs) | [cache](https://crates.io/crates/cache) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/krl/cache/1?label=krl%2fcache%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/krl/cache/issues/1) | Not Reported Yet |
| [0031](poc/0031-abox.rs) | [abox](https://crates.io/crates/abox) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/SonicFrog/abox/1?label=SonicFrog%2fabox%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/SonicFrog/abox/issues/1) | Not Reported Yet |
| [0032](poc/0032-conqueue.rs) | [conqueue](https://crates.io/crates/conqueue) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/longshorej/conqueue/9?label=longshorej%2fconqueue%239&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/longshorej/conqueue/issues/9) | Not Reported Yet |
| [0033](poc/0033-hashconsing.rs) | [hashconsing](https://crates.io/crates/hashconsing) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/AdrienChampion/hashconsing/1?label=AdrienChampion%2fhashconsing%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/AdrienChampion/hashconsing/issues/1) | Not Reported Yet |
| [0034](poc/0034-model.rs) | [model](https://crates.io/crates/model) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/spacejam/model/3?label=spacejam%2fmodel%233&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/spacejam/model/issues/3) | Not Reported Yet |
| [0035](poc/0035-late-static.rs) | [late-static](https://crates.io/crates/late-static) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Richard-W/late-static/1?label=Richard-W%2flate-static%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Richard-W/late-static/issues/1) | Not Reported Yet |
| [0036](poc/0036-bunch.rs) | [bunch](https://crates.io/crates/bunch) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/krl/bunch/1?label=krl%2fbunch%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/krl/bunch/issues/1) | Not Reported Yet |
| [0037](poc/0037-concread.rs) | [concread](https://crates.io/crates/concread) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/kanidm/concread/48?label=kanidm%2fconcread%2348&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/kanidm/concread/issues/48) | [![RUSTSEC-2020-0092](https://img.shields.io/badge/RUSTSEC-2020--0092-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0092.html) |
| [0038](poc/0038-parc.rs) | [parc](https://crates.io/crates/parc) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/pulls/detail/state/hyyking/rustracts/6?label=hyyking%2frustracts%236&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/hyyking/rustracts/pull/6) | Not Reported Yet |
| [0039](poc/0039-rcu_cell.rs) | [rcu_cell](https://crates.io/crates/rcu_cell) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Xudong-Huang/rcu_cell/3?label=Xudong-Huang%2frcu_cell%233&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Xudong-Huang/rcu_cell/issues/3) | Not Reported Yet |
| [0040](poc/0040-appendix.rs) | [appendix](https://crates.io/crates/appendix) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/krl/appendix/6?label=krl%2fappendix%236&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/krl/appendix/issues/6) | Not Reported Yet |
| [0041](poc/0041-unicycle.rs) | [unicycle](https://crates.io/crates/unicycle) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/udoprog/unicycle/8?label=udoprog%2funicycle%238&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/udoprog/unicycle/issues/8) | Not Reported Yet |
| [0042](poc/0042-toolshed.rs) | [toolshed](https://crates.io/crates/toolshed) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/ratel-rust/toolshed/12?label=ratel-rust%2ftoolshed%2312&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/ratel-rust/toolshed/issues/12) | Not Reported Yet |
| [0043](poc/0043-scottqueue.rs) | [scottqueue](https://crates.io/crates/scottqueue) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rossdylan/rust-scottqueue/1?label=rossdylan%2frust-scottqueue%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rossdylan/rust-scottqueue/issues/1) | Not Reported Yet |
| [0044](poc/0044-signal-simple.rs) | [signal-simple](https://crates.io/crates/signal-simple) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/kitsuneninetails/signal-rust/2?label=kitsuneninetails%2fsignal-rust%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/kitsuneninetails/signal-rust/issues/2) | Not Reported Yet |
| [0045](poc/0045-ruspiro-singleton.rs) | [ruspiro-singleton](https://crates.io/crates/ruspiro-singleton) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/RusPiRo/ruspiro-singleton/10?label=RusPiRo%2fruspiro-singleton%2310&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/RusPiRo/ruspiro-singleton/issues/10) | Not Reported Yet |
| [0046](poc/0046-generator.rs) | [generator](https://crates.io/crates/generator) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Xudong-Huang/generator-rs/27?label=Xudong-Huang%2fgenerator-rs%2327&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Xudong-Huang/generator-rs/issues/27) | Not Reported Yet |
| [0047](poc/0047-try-mutex.rs) | [try-mutex](https://crates.io/crates/try-mutex) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/mpdn/try-mutex/2?label=mpdn%2ftry-mutex%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/mpdn/try-mutex/issues/2) | [![RUSTSEC-2020-0087](https://img.shields.io/badge/RUSTSEC-2020--0087-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0087.html) |
| [0048](poc/0048-ticketed_lock.rs) | [ticketed_lock](https://crates.io/crates/ticketed_lock) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/kvark/ticketed_lock/7?label=kvark%2fticketed_lock%237&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/kvark/ticketed_lock/issues/7) | Not Reported Yet |
| [0049](poc/0049-slock.rs) | [slock](https://crates.io/crates/slock) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/BrokenLamp/slock-rs/2?label=BrokenLamp%2fslock-rs%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/BrokenLamp/slock-rs/issues/2) | Not Reported Yet |
| [0050](poc/0050-magnetic.rs) | [magnetic](https://crates.io/crates/magnetic) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/johnshaw/magnetic/9?label=johnshaw%2fmagnetic%239&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/johnshaw/magnetic/issues/9) | [![RUSTSEC-2020-0088](https://img.shields.io/badge/RUSTSEC-2020--0088-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0088.html) |
| [0051](poc/0051-syncpool.rs) | [syncpool](https://crates.io/crates/syncpool) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Chopinsky/byte_buffer/2?label=Chopinsky%2fbyte_buffer%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Chopinsky/byte_buffer/issues/2) | Not Reported Yet |
| [0052](poc/0052-reffers.rs) | [reffers](https://crates.io/crates/reffers) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/diwic/reffers-rs/7?label=diwic%2freffers-rs%237&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/diwic/reffers-rs/issues/7) | [![GitHub pull request detail](https://img.shields.io/github/pulls/detail/state/RustSec/advisory-db/533?style=flat-square)](https://github.com/RustSec/advisory-db/pull/533) |
| [0053](poc/0053-bottle.rs) | [bottle](https://crates.io/crates/bottle) | - manual<br>- SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/timothee-haudebourg/bottle/1?label=timothee-haudebourg%2fbottle%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/timothee-haudebourg/bottle/issues/1) | Not Reported Yet |
| [0054](poc/0054-tiny_future.rs) | [tiny_future](https://crates.io/crates/tiny_future) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/KizzyCode/tiny_future/1?label=KizzyCode%2ftiny_future%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/KizzyCode/tiny_future/issues/1) | Not Reported Yet |
| [0055](poc/0055-thex.rs) | [thex](https://crates.io/crates/thex) | - SendSyncChecker | N/A | [![RUSTSEC-2020-0090](https://img.shields.io/badge/RUSTSEC-2020--0090-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0090.html) |
| [0056](poc/0056-gfwx.rs) | [gfwx](https://crates.io/crates/gfwx) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Devolutions/gfwx-rs/7?label=Devolutions%2fgfwx-rs%237&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Devolutions/gfwx-rs/issues/7) | Not Reported Yet |
| [0057](poc/0057-async-coap.rs) | [async-coap](https://crates.io/crates/async-coap) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/google/rust-async-coap/33?label=google%2frust-async-coap%2333&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/google/rust-async-coap/issues/33) | Not Reported Yet |
| [0058](poc/0058-dces.rs) | [dces](https://crates.io/crates/dces) | - SendSyncChecker | [![GitLab issue](https://img.shields.io/badge/redox--os%2fdces--rust%238-grey?logo=GitLab&style=flat-square)](https://gitlab.redox-os.org/redox-os/dces-rust/-/issues/8) | Not Reported Yet |
| [0059](poc/0059-arc-swap.rs) | [arc-swap](https://crates.io/crates/arc-swap) | - manual<br>- SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/vorner/arc-swap/45?label=vorner%2farc-swap%2345&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/vorner/arc-swap/issues/45) | [![RUSTSEC-2020-0091](https://img.shields.io/badge/RUSTSEC-2020--0091-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0091.html) |
| [0060](poc/0060-noise_search.rs) | [noise_search](https://crates.io/crates/noise_search) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/pipedown/noise/72?label=pipedown%2fnoise%2372&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/pipedown/noise/issues/72) | Not Reported Yet |
| [0061](poc/0061-aovec.rs) | [aovec](https://crates.io/crates/aovec) | - SendSyncChecker | N/A | [![GitHub pull request detail](https://img.shields.io/github/pulls/detail/state/RustSec/advisory-db/528?style=flat-square)](https://github.com/RustSec/advisory-db/pull/528) |
| [0062](poc/0062-cgc.rs) | [cgc](https://crates.io/crates/cgc) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/playXE/cgc/5?label=playXE%2fcgc%235&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/playXE/cgc/issues/5) | Not Reported Yet |
| [0063](poc/0063-xcb.rs) | [xcb](https://crates.io/crates/xcb) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rtbo/rust-xcb/93?label=rtbo%2frust-xcb%2393&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rtbo/rust-xcb/issues/93) | Not Reported Yet |
| [0064](poc/0064-disrustor.rs) | [disrustor](https://crates.io/crates/disrustor) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/sklose/disrustor/1?label=sklose%2fdisrustor%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/sklose/disrustor/issues/1) | Not Reported Yet |
| [0065](poc/0065-v9.rs) | [v9](https://crates.io/crates/v9) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/purpleposeidon/v9/1?label=purpleposeidon%2fv9%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/purpleposeidon/v9/issues/1) | Not Reported Yet |
| [0066](poc/0066-kekbit.rs) | [kekbit](https://crates.io/crates/kekbit) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/motoras/kekbit/34?label=motoras%2fkekbit%2334&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/motoras/kekbit/issues/34) | Not Reported Yet |
| [0067](poc/0067-max7301.rs) | [max7301](https://crates.io/crates/max7301) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/edarc/max7301/1?label=edarc%2fmax7301%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/edarc/max7301/issues/1) | Not Reported Yet |
| [0068](poc/0068-buttplug.rs) | [buttplug](https://crates.io/crates/buttplug) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/buttplugio/buttplug-rs/225?label=buttplugio%2fbuttplug-rs%23225&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/buttplugio/buttplug-rs/issues/225) | Not Reported Yet |
| [0069](poc/0069-rusb.rs) | [rusb](https://crates.io/crates/rusb) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/a1ien/rusb/44?label=a1ien%2frusb%2344&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/a1ien/rusb/issues/44) | Not Reported Yet |
| [0070](poc/0070-multiqueue2.rs) | [multiqueue2](https://crates.io/crates/multiqueue2) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/abbychau/multiqueue2/10?label=abbychau%2fmultiqueue2%2310&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/abbychau/multiqueue2/issues/10) | Not Reported Yet |
| [0071](poc/0071-eventio.rs) | [eventio](https://crates.io/crates/eventio) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/petabi/eventio/33?label=petabi%2feventio%2333&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/petabi/eventio/issues/33) | Not Reported Yet |
| [0072](poc/0072-tensorflow.rs) | [tensorflow](https://crates.io/crates/tensorflow) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/tensorflow/rust/284?label=tensorflow%2frust%23284&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/tensorflow/rust/issues/284) | Not Reported Yet |
| [0073](poc/0073-stderr.rs) | [stderr](https://crates.io/crates/stderr) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/biluohc/stderr/5?label=biluohc%2fstderr%235&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/biluohc/stderr/issues/5) | Not Reported Yet |
| [0074](poc/0074-conquer_once.rs) | [conquer-once](https://crates.io/crates/conquer-once) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/oliver-giersch/conquer-once/3?label=oliver-giersch%2fconquer-once%233&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/oliver-giersch/conquer-once/issues/3) | Not Reported Yet |
| [0075](poc/0075-shine-stdext.rs) | [shine-stdext](https://crates.io/crates/shine-stdext) | - SendSyncChecker<br>- manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/gzp-crey/shine/1?label=gzp-crey%2fshine%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/gzp-crey/shine/issues/1) | Not Reported Yet |
| [0076](poc/0076-shine-store.rs) | [shine-store](https://crates.io/crates/shine-store) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/gzp-crey/shine/2?label=gzp-crey%2fshine%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/gzp-crey/shine/issues/2) | Not Reported Yet |
| [0077](poc/0077-va-ts.rs) | [va-ts](https://crates.io/crates/va-ts) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/video-audio/va-ts/4?label=video-audio%2fva-ts%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/video-audio/va-ts/issues/4) | Not Reported Yet |
| [0078](poc/0078-abi_stable.rs) | [abi_stable](https://crates.io/crates/abi_stable) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rodrimati1992/abi_stable_crates/44?label=rodrimati1992%2fabi_stable_crates%2344&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rodrimati1992/abi_stable_crates/issues/44) | Not Reported Yet |
| [0079](poc/0079-acc_reader.rs) | [acc_reader](https://crates.io/crates/acc_reader) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/netvl/acc_reader/1?label=netvl%2facc_reader%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/netvl/acc_reader/issues/1) | Not Reported Yet |
| [0080](poc/0080-bite.rs) | [bite](https://crates.io/crates/bite) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/hinaria/bite/1?label=hinaria%2fbite%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/hinaria/bite/issues/1) | Not Reported Yet |
| [0081](poc/0081-buffoon.rs) | [buffoon](https://crates.io/crates/buffoon) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/carllerche/buffoon/2?label=carllerche%2fbuffoon%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/carllerche/buffoon/issues/2) | Not Reported Yet |
| [0082](poc/0082-array_iterator.rs) | [array_iterator](https://crates.io/crates/array_iterator) | - manual<br>- PanicSafety | [![GitLab issue](https://img.shields.io/badge/kevincox%2farray_iterator.rs%231-grey?logo=GitLab&style=flat-square)](https://gitlab.com/kevincox/array_iterator.rs/-/issues/1) | Not Reported Yet |
| [0083](poc/0083-array-tools.rs) | [array-tools](https://crates.io/crates/array-tools) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/L117/array-tools/2?label=L117%2farray-tools%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/L117/array-tools/issues/2) | Not Reported Yet |
| [0084](poc/0084-autorand.rs) | [autorand](https://crates.io/crates/autorand) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/mersinvald/autorand-rs/5?label=mersinvald%2fautorand-rs%235&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/mersinvald/autorand-rs/issues/5) | Not Reported Yet |
| [0085](poc/0085-cdr.rs) | [cdr](https://crates.io/crates/cdr) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/hrektts/cdr-rs/10?label=hrektts%2fcdr-rs%2310&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/hrektts/cdr-rs/issues/10) | Not Reported Yet |
| [0086](poc/0086-bra.rs) | [bra](https://crates.io/crates/bra) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Enet4/bra-rs/1?label=Enet4%2fbra-rs%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Enet4/bra-rs/issues/1) | Not Reported Yet |
| [0087](poc/0087-bronzedb-protocol.rs) | [bronzedb-protocol](https://crates.io/crates/bronzedb-protocol) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Hexilee/BronzeDB/1?label=Hexilee%2fBronzeDB%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Hexilee/BronzeDB/issues/1) | Not Reported Yet |
| [0088](poc/0088-binjs_io.rs) | [binjs_io](https://crates.io/crates/binjs_io) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/binast/binjs-ref/460?label=binast%2fbinjs-ref%23460&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/binast/binjs-ref/issues/460) | Not Reported Yet |
| [0089](poc/0089-fil-ocl.rs) | [fil-ocl](https://crates.io/crates/fil-ocl) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/cogciprocate/ocl/194?label=cogciprocate%2focl%23194&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/cogciprocate/ocl/issues/194) | Not Reported Yet |
| [0090](poc/0090-endian.rs) | [endian_trait](https://crates.io/crates/endian_trait) | - PanicSafety | [![GitLab issue](https://img.shields.io/badge/myrrlyn%2fendian_trait%231-grey?logo=GitLab&style=flat-square)](https://gitlab.com/myrrlyn/endian_trait/-/issues/1) | Not Reported Yet |
| [0091](poc/0091-cassandra-proto.rs) | [cassandra-proto](https://crates.io/crates/cassandra-proto) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/AlexPikalov/cassandra-proto/3?label=AlexPikalov%2fcassandra-proto%233&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/AlexPikalov/cassandra-proto/issues/3) | Not Reported Yet |
| [0092](poc/0092-csv-sniffer.rs) | [csv-sniffer](https://crates.io/crates/csv-sniffer) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/jblondin/csv-sniffer/1?label=jblondin%2fcsv-sniffer%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/jblondin/csv-sniffer/issues/1) | Not Reported Yet |
| [0093](poc/0093-glium.rs) | [glium](https://crates.io/crates/glium) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/glium/glium/1907?label=glium%2fglium%231907&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/glium/glium/issues/1907) | Not Reported Yet |
| [0094](poc/0094-foreignc.rs) | [foreignc](https://crates.io/crates/foreignc) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/mart368b/foreignc/1?label=mart368b%2fforeignc%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/mart368b/foreignc/issues/1) | Not Reported Yet |
| [0095](poc/0095-calamine.rs) | [calamine](https://crates.io/crates/calamine) | - PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/tafia/calamine/199?label=tafia%2fcalamine%23199&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/tafia/calamine/issues/199) | Not Reported Yet |
| [0096](poc/0096-av-data.rs) | [av-data](https://crates.io/crates/av-data) | - manual<br>- PanicSafety | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rust-av/rust-av/136?label=rust-av%2frust-av%23136&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rust-av/rust-av/issues/136) | Not Reported Yet |

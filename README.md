# Rudra-PoC

This repository contains the list of memory safety and soundness bugs found during the Rudra project.

Contributors: See [REPORTING.md](./REPORTING.md) for the reporting guideline.

| ID | Crate | Method | Issue Report | RustSec ID |
| -- | ----- | ------ | ------------ | ---------- |
| 0000 | [rulinalg](https://crates.io/crates/rulinalg) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/AtheMathmo/rulinalg/201?label=AtheMathmo%2frulinalg%23201&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/AtheMathmo/rulinalg/issues/201) | [![RUSTSEC-2020-0023](https://img.shields.io/badge/RUSTSEC-2020--0023-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0023.html) |
| 0001 | [http](https://crates.io/crates/http) | - manual | hyperium/http#353 and hyperium/http#354 | [![RUSTSEC-2019-0034](https://img.shields.io/badge/RUSTSEC-2019--0034-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2019-0034.html) |
| 0002 | [http](https://crates.io/crates/http) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/hyperium/http/352?label=hyperium%2fhttp%23352&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/hyperium/http/issues/352) | [![RUSTSEC-2019-0033](https://img.shields.io/badge/RUSTSEC-2019--0033-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2019-0033.html) |
| 0003 | [ozone](https://crates.io/crates/ozone) | - manual | N/A | [![RUSTSEC-2020-0022](https://img.shields.io/badge/RUSTSEC-2020--0022-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0022.html) |
| 0004 | [rocket](https://crates.io/crates/rocket) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/SergioBenitez/Rocket/1312?label=SergioBenitez%2fRocket%231312&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/SergioBenitez/Rocket/issues/1312) | [![RUSTSEC-2020-0028](https://img.shields.io/badge/RUSTSEC-2020--0028-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0028.html) |
| 0005 | [failure](https://crates.io/crates/failure) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rust-lang-nursery/failure/336?label=rust-lang-nursery%2ffailure%23336&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rust-lang-nursery/failure/issues/336) | [![RUSTSEC-2019-0036](https://img.shields.io/badge/RUSTSEC-2019--0036-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2019-0036.html) |
| 0006 | [alpm-rs](https://crates.io/crates/alpm-rs) | - UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/pigeonhands/rust-arch/2?label=pigeonhands%2frust-arch%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/pigeonhands/rust-arch/issues/2) | [![RUSTSEC-2020-0032](https://img.shields.io/badge/RUSTSEC-2020--0032-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0032.html) |
| 0007 | [alg_ds](https://crates.io/crates/alg_ds) | - UnsafeDestructor | [![GitLab issue](https://img.shields.io/badge/dvshapkin%2falg--ds%231-grey?logo=GitLab&style=flat-square)](https://gitlab.com/dvshapkin/alg-ds/-/issues/1) | [![RUSTSEC-2020-0033](https://img.shields.io/badge/RUSTSEC-2020--0033-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0033.html) |
| 0008 | [arr](https://crates.io/crates/arr) | - manual<br>- UnsafeDestructor<br>- SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/sjep/array/1?label=sjep%2farray%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/sjep/array/issues/1) | [![RUSTSEC-2020-0034](https://img.shields.io/badge/RUSTSEC-2020--0034-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0034.html) |
| 0009 | [chunky](https://crates.io/crates/chunky) | - manual<br>- UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/aeplay/chunky/2?label=aeplay%2fchunky%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/aeplay/chunky/issues/2) | [![RUSTSEC-2020-0035](https://img.shields.io/badge/RUSTSEC-2020--0035-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0035.html) |
| 0010 | [crayon](https://crates.io/crates/crayon) | - manual<br>- UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/shawnscode/crayon/87?label=shawnscode%2fcrayon%2387&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/shawnscode/crayon/issues/87) | [![RUSTSEC-2020-0037](https://img.shields.io/badge/RUSTSEC-2020--0037-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0037.html) |
| 0011 | [obstack](https://crates.io/crates/obstack) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/petertodd/rust-obstack/4?label=petertodd%2frust-obstack%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/petertodd/rust-obstack/issues/4) | [![RUSTSEC-2020-0040](https://img.shields.io/badge/RUSTSEC-2020--0040-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0040.html) |
| 0012 | [ordnung](https://crates.io/crates/ordnung) | - manual<br>- UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/maciejhirsz/ordnung/8?label=maciejhirsz%2fordnung%238&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/maciejhirsz/ordnung/issues/8) | [![RUSTSEC-2020-0038](https://img.shields.io/badge/RUSTSEC-2020--0038-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0038.html) |
| 0013 | [simple-slab](https://crates.io/crates/simple-slab) | - UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/nathansizemore/simple-slab/2?label=nathansizemore%2fsimple-slab%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/nathansizemore/simple-slab/issues/2) | [![RUSTSEC-2020-0039](https://img.shields.io/badge/RUSTSEC-2020--0039-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0039.html) |
| 0014 | [sized-chunks](https://crates.io/crates/sized-chunks) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/bodil/sized-chunks/11?label=bodil%2fsized-chunks%2311&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/bodil/sized-chunks/issues/11) | [![RUSTSEC-2020-0041](https://img.shields.io/badge/RUSTSEC-2020--0041-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0041.html) |
| 0015 | [atom](https://crates.io/crates/atom) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/slide-rs/atom/13?label=slide-rs%2fatom%2313&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/slide-rs/atom/issues/13) | [![RUSTSEC-2020-0044](https://img.shields.io/badge/RUSTSEC-2020--0044-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0044.html) |
| 0016 | [stack](https://crates.io/crates/stack) | - UnsafeDestructor | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/arcnmx/stack-rs/4?label=arcnmx%2fstack-rs%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/arcnmx/stack-rs/issues/4) | [![RUSTSEC-2020-0042](https://img.shields.io/badge/RUSTSEC-2020--0042-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0042.html) |
| 0017 | [array-queue](https://crates.io/crates/array-queue) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/raviqqe/array-queue/2?label=raviqqe%2farray-queue%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/raviqqe/array-queue/issues/2) | [![RUSTSEC-2020-0047](https://img.shields.io/badge/RUSTSEC-2020--0047-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0047.html) |
| 0018 | [dync](https://crates.io/crates/dync) | - manual | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/elrnv/dync/4?label=elrnv%2fdync%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/elrnv/dync/issues/4) | [![RUSTSEC-2020-0050](https://img.shields.io/badge/RUSTSEC-2020--0050-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0050.html) |
| 0019 | [futures](https://crates.io/crates/futures) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rust-lang/futures-rs/2239?label=rust-lang%2ffutures-rs%232239&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rust-lang/futures-rs/issues/2239) | [![RUSTSEC-2020-0059](https://img.shields.io/badge/RUSTSEC-2020--0059-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0059.html) |
| 0020 | [beef](https://crates.io/crates/beef) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/maciejhirsz/beef/37?label=maciejhirsz%2fbeef%2337&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/maciejhirsz/beef/issues/37) | Not Reported Yet |
| 0021 | [futures-intrusive](https://crates.io/crates/futures-intrusive) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Matthias247/futures-intrusive/53?label=Matthias247%2ffutures-intrusive%2353&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Matthias247/futures-intrusive/issues/53) | [![RUSTSEC-2020-0072](https://img.shields.io/badge/RUSTSEC-2020--0072-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0072.html) |
| 0022 | [atomic-option](https://crates.io/crates/atomic-option) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/reem/rust-atomic-option/4?label=reem%2frust-atomic-option%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/reem/rust-atomic-option/issues/4) | Not Reported Yet |
| 0023 | [convec](https://crates.io/crates/convec) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/krl/convec/2?label=krl%2fconvec%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/krl/convec/issues/2) | Not Reported Yet |
| 0024 | [lock_api](https://crates.io/crates/lock_api) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Amanieu/parking_lot/258?label=Amanieu%2fparking_lot%23258&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Amanieu/parking_lot/issues/258) | [![RUSTSEC-2020-0070](https://img.shields.io/badge/RUSTSEC-2020--0070-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0070.html) |
| 0025 | [im](https://crates.io/crates/im) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/bodil/im-rs/157?label=bodil%2fim-rs%23157&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/bodil/im-rs/issues/157) | Not Reported Yet |
| 0026 | [may_queue](https://crates.io/crates/may_queue) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Xudong-Huang/may/88?label=Xudong-Huang%2fmay%2388&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Xudong-Huang/may/issues/88) | Not Reported Yet |
| 0027 | [libsbc](https://crates.io/crates/libsbc) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/mvertescher/libsbc-rs/4?label=mvertescher%2flibsbc-rs%234&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/mvertescher/libsbc-rs/issues/4) | Not Reported Yet |
| 0028 | [lever](https://crates.io/crates/lever) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/vertexclique/lever/15?label=vertexclique%2flever%2315&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/vertexclique/lever/issues/15) | Not Reported Yet |
| 0029 | [lexer](https://crates.io/crates/lexer) | - SendSyncChecker | [![GitLab issue](https://img.shields.io/badge/nathanfaucett%2frs--lexer%232-grey?logo=GitLab&style=flat-square)](https://gitlab.com/nathanfaucett/rs-lexer/-/issues/2) | Not Reported Yet |
| 0030 | [cache](https://crates.io/crates/cache) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/krl/cache/1?label=krl%2fcache%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/krl/cache/issues/1) | Not Reported Yet |
| 0031 | [abox](https://crates.io/crates/abox) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/SonicFrog/abox/1?label=SonicFrog%2fabox%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/SonicFrog/abox/issues/1) | Not Reported Yet |
| 0032 | [conqueue](https://crates.io/crates/conqueue) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/longshorej/conqueue/9?label=longshorej%2fconqueue%239&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/longshorej/conqueue/issues/9) | Not Reported Yet |
| 0033 | [hashconsing](https://crates.io/crates/hashconsing) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/AdrienChampion/hashconsing/1?label=AdrienChampion%2fhashconsing%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/AdrienChampion/hashconsing/issues/1) | Not Reported Yet |
| 0034 | [model](https://crates.io/crates/model) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/spacejam/model/3?label=spacejam%2fmodel%233&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/spacejam/model/issues/3) | Not Reported Yet |
| 0035 | [late-static](https://crates.io/crates/late-static) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Richard-W/late-static/1?label=Richard-W%2flate-static%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Richard-W/late-static/issues/1) | Not Reported Yet |
| 0036 | [bunch](https://crates.io/crates/bunch) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/krl/bunch/1?label=krl%2fbunch%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/krl/bunch/issues/1) | Not Reported Yet |
| 0037 | [concread](https://crates.io/crates/concread) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/kanidm/concread/48?label=kanidm%2fconcread%2348&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/kanidm/concread/issues/48) | Not Reported Yet |
| 0038 | [parc](https://crates.io/crates/parc) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/pulls/detail/state/hyyking/rustracts/6?label=hyyking%2frustracts%236&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/hyyking/rustracts/pull/6) | Not Reported Yet |
| 0039 | [rcu_cell](https://crates.io/crates/rcu_cell) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Xudong-Huang/rcu_cell/3?label=Xudong-Huang%2frcu_cell%233&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Xudong-Huang/rcu_cell/issues/3) | Not Reported Yet |
| 0040 | [appendix](https://crates.io/crates/appendix) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/krl/appendix/6?label=krl%2fappendix%236&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/krl/appendix/issues/6) | Not Reported Yet |
| 0041 | [unicycle](https://crates.io/crates/unicycle) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/udoprog/unicycle/8?label=udoprog%2funicycle%238&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/udoprog/unicycle/issues/8) | Not Reported Yet |
| 0042 | [toolshed](https://crates.io/crates/toolshed) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/ratel-rust/toolshed/12?label=ratel-rust%2ftoolshed%2312&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/ratel-rust/toolshed/issues/12) | Not Reported Yet |
| 0043 | [scottqueue](https://crates.io/crates/scottqueue) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/rossdylan/rust-scottqueue/1?label=rossdylan%2frust-scottqueue%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/rossdylan/rust-scottqueue/issues/1) | Not Reported Yet |
| 0044 | [signal-simple](https://crates.io/crates/signal-simple) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/kitsuneninetails/signal-rust/2?label=kitsuneninetails%2fsignal-rust%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/kitsuneninetails/signal-rust/issues/2) | Not Reported Yet |
| 0045 | [ruspiro-singleton](https://crates.io/crates/ruspiro-singleton) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/RusPiRo/ruspiro-singleton/10?label=RusPiRo%2fruspiro-singleton%2310&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/RusPiRo/ruspiro-singleton/issues/10) | Not Reported Yet |
| 0046 | [generator](https://crates.io/crates/generator) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Xudong-Huang/generator-rs/27?label=Xudong-Huang%2fgenerator-rs%2327&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Xudong-Huang/generator-rs/issues/27) | Not Reported Yet |
| 0047 | [try-mutex](https://crates.io/crates/try-mutex) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/mpdn/try-mutex/2?label=mpdn%2ftry-mutex%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/mpdn/try-mutex/issues/2) | [![RUSTSEC-2020-0087](https://img.shields.io/badge/RUSTSEC-2020--0087-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0087.html) |
| 0048 | [ticketed_lock](https://crates.io/crates/ticketed_lock) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/kvark/ticketed_lock/7?label=kvark%2fticketed_lock%237&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/kvark/ticketed_lock/issues/7) | Not Reported Yet |
| 0049 | [slock](https://crates.io/crates/slock) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/BrokenLamp/slock-rs/2?label=BrokenLamp%2fslock-rs%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/BrokenLamp/slock-rs/issues/2) | Not Reported Yet |
| 0050 | [magnetic](https://crates.io/crates/magnetic) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/johnshaw/magnetic/9?label=johnshaw%2fmagnetic%239&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/johnshaw/magnetic/issues/9) | [![RUSTSEC-2020-0088](https://img.shields.io/badge/RUSTSEC-2020--0088-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0088.html) |
| 0051 | [syncpool](https://crates.io/crates/syncpool) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Chopinsky/byte_buffer/2?label=Chopinsky%2fbyte_buffer%232&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Chopinsky/byte_buffer/issues/2) | Not Reported Yet |
| 0052 | [reffers](https://crates.io/crates/reffers) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/diwic/reffers-rs/7?label=diwic%2freffers-rs%237&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/diwic/reffers-rs/issues/7) | Not Reported Yet |
| 0053 | [bottle](https://crates.io/crates/bottle) | - manual<br>- SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/timothee-haudebourg/bottle/1?label=timothee-haudebourg%2fbottle%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/timothee-haudebourg/bottle/issues/1) | Not Reported Yet |
| 0054 | [tiny_future](https://crates.io/crates/tiny_future) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/KizzyCode/tiny_future/1?label=KizzyCode%2ftiny_future%231&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/KizzyCode/tiny_future/issues/1) | Not Reported Yet |
| 0055 | [thex](https://crates.io/crates/thex) | - SendSyncChecker | N/A | [![RUSTSEC-2020-0090](https://img.shields.io/badge/RUSTSEC-2020--0090-blue?style=flat-square)](https://rustsec.org/advisories/RUSTSEC-2020-0090.html) |
| 0056 | [gfwx](https://crates.io/crates/gfwx) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/Devolutions/gfwx-rs/7?label=Devolutions%2fgfwx-rs%237&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/Devolutions/gfwx-rs/issues/7) | Not Reported Yet |
| 0057 | [async-coap](https://crates.io/crates/async-coap) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/google/rust-async-coap/33?label=google%2frust-async-coap%2333&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/google/rust-async-coap/issues/33) | Not Reported Yet |
| 0058 | [dces](https://crates.io/crates/dces) | - SendSyncChecker | https://gitlab.redox-os.org/redox-os/dces-rust/-/issues/8 | Not Reported Yet |
| 0059 | [arc-swap](https://crates.io/crates/arc-swap) | - manual<br>- SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/vorner/arc-swap/45?label=vorner%2farc-swap%2345&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/vorner/arc-swap/issues/45) | Not Reported Yet |
| 0060 | [noise_search](https://crates.io/crates/noise_search) | - SendSyncChecker | [![GitHub issue or PR](https://img.shields.io/github/issues/detail/state/pipedown/noise/72?label=pipedown%2fnoise%2372&logo=GitHub&cacheSeconds=3600&style=flat-square)](https://github.com/pipedown/noise/issues/72) | Not Reported Yet |

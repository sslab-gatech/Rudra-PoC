# Reporting Guide

https://github.com/joeyespo/grip

## Install the Reporting Utility

- [sccache](https://github.com/mozilla/sccache)
  - Technically this part can be optional, but I was lazy.
  - Installation is quite simple. Download sccache binary from their [relases page](https://github.com/mozilla/sccache/releases) and put it on `PATH`.
- Copy `config.toml.template` to `config.toml` and fill it with your own data.
- Run `sudo ./setup.sh` to install dependencies. (TODO: check if it is still necessary)
- Run `cargo install --path cli`. This installs `rudra-poc` binary.
  - This program is working directory-agnostic. It remembers the location of `rudra-poc` project when compiling.

## Metadata Format

At the beginning of each poc, there is a comment that contains metadata for the poc as TOML format.

The format of the metadata is as follows:
````
/*!
```rudra-poc
[target]
(...)

[test]
(...)

[report]
(...)
```
!*/
````

### target

- **crate**: (string) the name of the target crate
- **version**: (string) the version of the target crate to test the poc
- **features**: (optional string array) list of features to enable (TODO: not supported yet)
- **peer**: (optional object array) peer dependencies, follows the same pattern with target. See `./poc/0024-lock_api.rs` for an example.
  - Ironically, `features` are supported here

### test

- **analyzers**: (string array) an array of analyzers that are expected to report bugs for this crate
  - The name of analyzers should match the corresponding implementation in Rudra. Otherwise, the parsing will fail.
  - Example: `["manual", "UnsafeDestructor"]`
- **cargo_flags**: (optional string array) Cargo flags that are appended to cargo commands, e.g., `["--release"]`
- **cargo_toolchain**: (optional string) Toolchain name that is appended to cargo commands, e.g., `"nightly"`

### report

- **issue_url**: (optional string) reported issue URL
- **issue_date**: (optional date) disclosure date, without quote, e.g., `2020-12-31`
- **rustsec_url**: (optional string) RustSec PR URL
- **rustsec_id**: (optional string) RustSec ID, e.g., `"RUSTSEC-2020-1234"`

Report section is empty when PoC is created. Fill each field as you make progress in reporting.

## Workflow

`rudra-poc` binary helps you create PoC for bugs and report them.

### 1. Writing PoC

```shell
rudra-poc add <crate> <version>
```

Use `add` subcommand to start working on a new PoC. This creates a PoC file under `poc` directory and populate `poc-debug` directory. Modify the source code to show how safe APIs in the crate can trigger undefined behaviors. Source files in `poc-debug` directory is symbolic linked, so you don't need to copy back and force.

There is a boilerplate module available that helps track the heap status and sectioning multiple vulnerabilities. If you want to use it, add the module definition `mod boilerplate` and call `boilerplate::init()` in main.


```shell
rudra-poc run <PoC ID>
```

Then, use `run` subcommand to verify the output. This executes the PoC program with `cargo run` command with `cargo_flags` and `cargo_toolchain` specified the metadata (you have to add them manually when you are working inside `poc-debug`). If `run` subcommand generates the desired output, proceed to report the issue. Note that you can use `rudra-poc run --debug <PoC ID>` to re-populate `poc-debug` directory with specific PoC.

### 2. Reporting PoC (WIP)

```shell
rudra-poc generate [issue|rustsec|rustsec-direct] <PoC ID>
rudra-poc report [issue|rustsec] <PoC ID>
```

(TODO)

## Tips

- Should I include `informational = "unsound"` or not?
  - Bugs that require *actively malicious clients* to trigger should contain `informational = "unsound"` line. This includes implementing traits on the client side that are supposed to be sealed, accessing `doc(hidden)` APIs, violating behavioral contracts such as comparator consistency, etc.
  - Bugs that require *uncommon but valid constructions* to trigger should probably get security advisories without `informational` line. This includes panicking iterators, omitting the drop call, types with internal mutability, types with large alignment, etc.
  - `informational = "unsound"` is sometimes described as "not a vulnerability." Therefore, we prefer to have an advisory without this line since our group is inclined much more towards classifying bugs in grey area as "vulnerabilities" than "not vulnerabilities." However, it seems that the community sentiment for unsound advisories is closer to "low severity vulnerabilities" than "not a vulnerability," so don't focus too much on the definition and spend time on arguing whether a bug should contain `informational` line or not. Just follow the guidance from RustSec maintainers and be respectful.
- Do not wait *too much* for the original author before filing an advisory. Waiting for the author's response is indeed a good courtesy, but filing an advisory in a responsive manner has other benefits. Also, many of bugs are from inactive crates where the author is often not very responsive.
- If an issue tracker is not available on the original repository, either (1) create a PR instead (if you have time), or (2) report the bug directly to RustSec.
- Don't feel guilty about filing RustSec advisory for unpopular crates. As per [maintainer's comment](https://github.com/RustSec/advisory-db/pull/328#issuecomment-653725642):
  > We don't generally limit advisories based on crate popularity -- if no one is using the crate, then no one will be bothered by it :-)"

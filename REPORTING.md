# Reporting Guide

Please do not create a merge commit when pulling remote update. Instead, use `git pull --rebase origin master`.

## Install the Reporting Utility

- [sccache](https://github.com/mozilla/sccache)
  - Technically this part can be optional, but I was lazy.
  - Installation is quite simple. Download sccache binary from their [relases page](https://github.com/mozilla/sccache/releases) and put it on `PATH`.
- Copy `config.toml.template` to `config.toml` and fill it with your own data.
- Run `sudo ./setup.sh` to install dependencies. (TODO: check if it is still necessary)
- Run `cargo install --path cli`. This installs `rudra-poc` binary.
  - This program is working directory agnostic. It remembers the location of `rudra-poc` project when compiling.
  - Don't forget to reinstall the program after fetching the remote.

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

[[bugs]]
(...)

[[bugs]]
(...)
```
!*/
````

### target

- **crate**: (string) the name of the target crate
- **version**: (string) the version of the target crate to test the poc
  - The latest version in the index
- **indexed_name**: (optional string) the name of the target crate in the index
  - This option is only needed when the crate is renamed or the bug was moved to another subcrate
- **indexed_version**: (optional string) the version of the target in the index
  - The latest version at the time of 2020-07-04
- **features**: (optional string array) list of features to enable (TODO: not supported yet)
- **peer**: (optional object array) peer dependencies, follows the same pattern with target. See `./poc/0024-lock_api.rs` for an example.
  - Ironically, `features` are supported here

### test (optional)

- **cargo_flags**: (optional string array) Cargo flags that are appended to cargo commands, e.g., `["--release"]`
- **cargo_toolchain**: (optional string) Toolchain name that is appended to cargo commands, e.g., `"nightly"`

### report

- **issue_url**: (optional string) reported issue URL
- **issue_date**: (optional date) disclosure date, without quote, e.g., `2020-12-31`
- **rustsec_url**: (optional string) RustSec PR URL
- **rustsec_id**: (optional string) RustSec ID, e.g., `"RUSTSEC-2020-1234"`

Report section is empty when PoC is created. Fill each field as you make progress in reporting.

### bugs

Uses [Array of Tables](https://toml.io/en/v1.0.0#array-of-tables) in TOML format.

- **analyzer**: (string) The name of the analyzer that found the bug. It should match the corresponding implementation in Rudra.
  - Example: `["Manual", "UnsafeDestructor", "SendSyncVariance", "UnsafeDataflow"]`
- **guide**: (optional string)
  - When the analyzer is "Manual" - Which analyzer guided you to audit this crate?
  - When the analyzer is not "Manual" - This value must be "Manual" if exists. The bug is in the location reported by the primary analyzer, but it didn't match the exact pattern the analyzer was looking for and required additional manual auditing.
- **bug_class**: (string) The bug class.
  - Example: `["SendSyncVariance", "UninitExposure", "HigherOrderInvariant", "PanicSafety", "Other"]`
- **bug_count**: (optional integer) Default to 1, number of bugs that correspond to this pattern.
- **rudra_report_locations**: (string array) The location of the bug in the `indexed_version`

## Unreported Metadata

Unreported directory tracks valid bugs that are not reported for various reasons.
It has very similar format to the poc metadata, but there are some minor differences.

### target

- **crate**: (string) the name of the target crate
- **version**: (string) the version of the target crate to test the poc
  - The latest version in the index

### bugs

Uses [Array of Tables](https://toml.io/en/v1.0.0#array-of-tables) in TOML format.

- **analyzer**: (string) The name of the analyzer that found the bug. It should match the corresponding implementation in Rudra.
  - "Manual" analyzer is not allowed
- **bug_class**: (string) The bug class.
  - Example: `["SendSyncVariance", "UninitExposure", "HigherOrderInvariant", "PanicSafety", "Other"]`
- **reason**: `["internal", "experimental", "unobservable", "independently fixed", "wasm only"]`
- **location**: (string) The location of the bug in the `indexed_version`

Note: no `bug_count`, `rudra_report_locations` (string array) -> `location` (string)

## Workflow

`rudra-poc` binary helps you create PoC for bugs and report them.

### 1. Writing PoC

```shell
rudra-poc add <crate> <version>
```

Use `add` subcommand to start working on a new PoC. This creates a PoC file under `poc` directory and populate `poc-debug` directory. Modify the source code to show how safe APIs in the crate can trigger undefined behaviors. Source files in `poc-debug` directory are symbolic links, so you don't need to copy files back and force to `poc` directory.

There is a boilerplate module available that helps track the heap status and section multiple vulnerabilities. If you want to use it, add the module definition `mod boilerplate` and call `boilerplate::init()` in main.


```shell
rudra-poc run <PoC ID>
```

Then, use `run` subcommand to verify the output. This executes the PoC program with `cargo run` command with `cargo_flags` and `cargo_toolchain` specified the metadata (you have to add them manually when you are working inside `poc-debug`). If `run` subcommand generates the desired output, proceed to report the issue.

Note that you can use `rudra-poc run --debug <PoC ID>` to re-populate `poc-debug` directory with specific PoC. You might need to run this after changing `cargo_flags` or `cargo_toolchain` in the metadata.

### 2. Reporting PoC

```shell
rudra-poc generate [issue|rustsec|rustsec-direct] <PoC ID>
```

`generate` subcommand generates issue templates for you. It will populate `advisory.md` and `issue_report.md` in the project directory. `issue_report.md` will contain the content of the issue/PR and `advisory.md` will contain the content of the RustSec advisory. Edit those files with your favorite markdown editor.

```shell
rudra-poc report [issue|rustsec] <PoC ID>
```

Then, use `report` subcommand to report the issue to the crate repository or RustSec advisory DB. `rudra-poc report issue <PoC ID>` first parses the repository URL from crates.io API, submits an issue based on the content of `issue_report.md`, updates the PoC metadata, and regenerates `README.md`. Currently, the automatic issue submission is only supported for GitHub repositories; you have to manually report on other sites such as GitLab or BitBucket.

When manually reporting,
1. Submit an issue to the issue tracker
2. Update `issue_url` and `issue_date` fields of the PoC metadata
3. Run `rudra-poc update`

Sometimes, the issue tracker for the crate is not available. You can either (1) create a PR instead of submitting an issue (if you have time), or (2) report the bug directly to RustSec. If you choose 1, follow the steps for a manual reporting. If you choose 2, use `rudra-poc generate rustsec-direct <PoC ID>` command for your convenience.

`rudra-poc report rustsec <PoC ID>` automatically creates a branch, push it to your fork, submits a pull request to RustSec advisory DB, updates the PoC metadata, and regenerates `README.md`. You can find the cloned repository under `advisory-db` directory under the project directory.

## Tips

- Should I include `informational = "unsound"` or not?
  - Bugs that require *actively malicious clients* to trigger should contain `informational = "unsound"` line. This includes implementing traits on the client side that are supposed to be sealed, violating behavioral contracts such as comparator consistency, etc.
  - Bugs that require *uncommon but valid constructions* to trigger should probably get security advisories without `informational` line. This includes panicking iterators, omitting the drop call, types with internal mutability, types with large alignment, etc.
  - `informational = "unsound"` is sometimes described as "not a vulnerability." Therefore, we prefer to have an advisory without this line since our group is inclined much more towards classifying bugs in grey area as "vulnerabilities" than "not vulnerabilities." However, it seems that the community sentiment for unsound advisories is closer to "low severity vulnerabilities" than "not a vulnerability," so don't focus too much on the definition and spend time on arguing whether a bug should contain `informational` line or not. Just follow the guidance from RustSec maintainers and be respectful.
- Do not wait *too much* for the original author before filing an advisory. Waiting for the author's response is indeed a good courtesy, but filing an advisory in a responsive manner has other benefits. Also, many of bugs are from inactive crates where the author is often not very responsive.
- Don't feel guilty about filing RustSec advisory for unpopular crates. As per [maintainer's comment](https://github.com/RustSec/advisory-db/pull/328#issuecomment-653725642):
  > We don't generally limit advisories based on crate popularity -- if no one is using the crate, then no one will be bothered by it :-)"

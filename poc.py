#!/usr/bin/env python3

import argparse
import datetime
import os
import re
import shutil
import subprocess
import requests
import sys
import tempfile
import toml

from subprocess import PIPE, STDOUT
from urllib.parse import urlparse

# https://man7.org/linux/man-pages/man7/signal.7.html
SIGNAL_MAP = {
    1: "SIGHUP",
    2: "SIGINT",
    3: "SIGQUIT",
    4: "SIGILL",
    5: "SIGTRAP",
    6: "SIGABRT",
    7: "SIGBUS",
    8: "SIGFPE",
    9: "SIGKILL",
    10: "SIGUSR1",
    11: "SIGSEGV",
    12: "SIGUSR2",
    13: "SIGPIPE",
    14: "SIGALRM",
    15: "SIGTERM",
    16: "SIGSTKFLT",
    17: "SIGCHLD",
    18: "SIGCONT",
    19: "SIGSTOP",
    20: "SIGTSTP",
    21: "SIGTTIN",
    22: "SIGTTOU",
    23: "SIGURG",
    24: "SIGXCPU",
    25: "SIGXFSZ",
    26: "SIGVTALRM",
    27: "SIGPROF",
    28: "SIGWINCH",
    29: "SIGIO",
    30: "SIGPWR",
    31: "SIGSYS",
}

parser = argparse.ArgumentParser()

subparsers = parser.add_subparsers(dest="cmd")

parser_add = subparsers.add_parser("add")
parser_add.add_argument("crate", help="target crate name")
parser_add.add_argument("version", help="target crate version")

parser_run = subparsers.add_parser("run")
parser_run.add_argument("id", help="poc ID (4 digits)")
parser_run.add_argument("--copy", action="store_true", help="saves the PoC directory to `poc-debug` if set")

parser_report = subparsers.add_parser("report")
parser_report.add_argument("id", help="poc ID (4 digits)")
parser_report.add_argument("--preview", action="store_true", help="prints the report content without reporting")
parser_report.add_argument("--crate_repo", action="store_true", help="reports the issue to the crate's repository")
parser_report.add_argument("--rustsec", action="store_true", help="reports the issue to RustSec advisory")

args = parser.parse_args()

# Read the configuration file
if not os.path.exists("config.toml"):
    print("`config.toml` does not exist")
    sys.exit(1)

with open("config.toml") as config_file:
    config_text = config_file.read()
    config = toml.loads(config_text)

user_name = config["name"]
user_email = config["email"]
user_github_id = config["github_id"]
user_token = config["token"]
rustsec_fork_url = config["rustsec_fork_url"]

GITHUB_CLIENT_HEADERS = {
    "Accept": "application/vnd.github.v3+json",
    "User-Agent": user_github_id,
    "Authorization": f"token {user_token}",
}

# Check the environment
os_version = subprocess.check_output(["lsb_release", "-sd"]).decode().strip()

# Prepare Git repository
if not os.path.exists("advisory-db"):
    os.system(f"git clone {rustsec_fork_url} advisory-db")
    os.system(f"git remote add rustsec https://github.com/RustSec/advisory-db.git")

# Map PoC number to PoC name
# Note that the name doesn't contain `.rs` extension
poc_id_to_name = {}

# Record known PoC
poc_dir_pattern = re.compile(r"(\d{4})-.+")

for name_with_ext in os.listdir("poc"):
    if os.path.isfile(f"poc/{name_with_ext}"):
        name = os.path.splitext(name_with_ext)[0]
        match = poc_dir_pattern.match(name)
        if match is not None:
            poc_id = match.group(1)
            poc_id_to_name[poc_id] = name

# Override rustc to enable the build cache
os.environ["RUSTC_WRAPPER"] = "sccache"

# Set LD_LIBRARY_PATH
link_path = os.path.abspath("dependencies")
if "LD_LIBRARY_PATH" in os.environ:
    os.environ["LD_LIBRARY_PATH"] += ":" + link_path
else:
    os.environ["LD_LIBRARY_PATH"] = link_path

# Set RUSTFLAGS
if "RUSTFLAGS" in os.environ:
    os.environ["RUSTFLAGS"] += " -A warnings"
else:
    os.environ["RUSTFLAGS"] = "-A warnings"


def read_metadata(poc_id):
    poc_name = poc_id_to_name[poc_id]

    with open(f"poc/{poc_name}.rs") as poc_file:
        lines = poc_file.readlines()
        if lines[0] == "/*!\n" and lines[1] == "```crux-poc\n":
            idx = lines.index("```\n")
            toml_str = ''.join(lines[2:idx])
            return toml.loads(toml_str)

    return None


# This method appends metadata to the last section, which is expected to be [report]
def append_metadata(poc_id, dict):
    poc_name = poc_id_to_name[poc_id]

    with open(f"poc/{poc_name}.rs") as poc_file:
        lines = poc_file.readlines()
        if lines[0] == "/*!\n" and lines[1] == "```crux-poc\n":
            idx = lines.index("```\n")
        else:
            raise Exception("PoC metadata comment not found")

    dict_toml = toml.dumps(dict)

    with open(f"poc/{poc_name}.rs", "w") as poc_file:
        poc_file.write("".join(lines[:idx]))
        poc_file.write(dict_toml)
        poc_file.write("".join(lines[idx:]))


def read_code(poc_id):
    poc_name = poc_id_to_name[poc_id]

    with open(f"poc/{poc_name}.rs") as poc_file:
        lines = poc_file.readlines()
        if lines[0] == "/*!\n" and lines[1] == "```crux-poc\n":
            idx = lines.index("!*/\n")
            return "".join(lines[idx+1:]).strip()

    raise Exception("Failed to read PoC code")


def prepare_report(poc_id):
    metadata = read_metadata(poc_id)
    code = read_code(poc_id)

    target_crate = metadata["target"]["crate"]
    target_version = metadata["target"]["version"]

    title = metadata["report"]["title"]
    description = metadata["report"]["description"]
    code_snippets = metadata["report"]["code_snippets"]

    cargo_flags_str = ""
    if "cargo_flags" in metadata["test"]:
        cargo_flags_str = "* Cargo flags: "
        cargo_flags_str += " ".join(metadata["test"]["cargo_flags"]) + "\n"

    # Add `--quiet` flag
    if "cargo_flags" in metadata["test"]:
        metadata["test"]["cargo_flags"].append("--quiet")
    else:
        metadata["test"]["cargo_flags"] = ["--quiet"]
    
    with tempfile.TemporaryDirectory() as tmpdir:
        prepare_cargo_dir(poc_id, tmpdir)

        # parse `rustc` version
        metadata["test"]["cargo_flags"] += ["--", "--version"]
        cmd = prepare_cargo_cmd(metadata, "rustc")
        rustc_version = subprocess.run(cmd, stdout=PIPE, cwd=tmpdir).stdout.decode().strip()
        metadata["test"]["cargo_flags"] = metadata["test"]["cargo_flags"][:-2]

        cmd = prepare_cargo_cmd(metadata, "run")
        exec_result = subprocess.run(cmd, stdout=PIPE, stderr=STDOUT, cwd=tmpdir)
        exec_output = exec_result.stdout.decode().strip()
        exit_code = exec_result.returncode

        # > A negative value `-N` indicates that the child was terminated by signal `N` (POSIX only).
        # https://docs.python.org/3.6/library/subprocess.html#subprocess.CompletedProcess.returncode
        if exit_code < 0:
            exit_code_str = f"{exit_code} ({SIGNAL_MAP[-exit_code]})"
        else:
            exit_code_str = str(exit_code)

    report_content = "Hello, we have noticed a soundness issue and/or a potential security vulnerability in this crate while performing a security scan on crates.io.\n\n"

    report_content += "".join(
        map(lambda s: s + "\n\n", code_snippets)
    )
    report_content += "# Description\n\n" + description
    report_content += f"""

# Demonstration

* Crate: {target_crate}
* Version: {target_version}
* OS: {os_version}
* Rust: {rustc_version}
{cargo_flags_str}
```rust
{code}
```

Output:
```
{exec_output}
```

Return Code: {exit_code_str}
"""

    return {
        "title": title,
        "code": code,
        "description": report_content,
    }


def prepare_cargo_dir(poc_id, cargo_dir):
    poc_name = poc_id_to_name[poc_id]
    metadata = read_metadata(poc_id)

    # Cargo.toml
    with open(f"{cargo_dir}/Cargo.toml", "w") as manifest_file:
        manifest_content = f"""[package]
name = "crux-poc-{poc_id}"
version = "0.1.0"
authors = ["{user_name} <{user_email}>"]
edition = "2018"
build = "build.rs"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
logging-allocator = "0.1.1"
log = "0.4"
{metadata["target"]["crate"]} = "={metadata["target"]["version"]}"
"""

        if "peer" in metadata["target"]:
            for crate in metadata["target"]["peer"]:
                manifest_content += f'''{crate["crate"]} = "={crate["version"]}"\n'''

        manifest_file.write(manifest_content)

    # build.rs
    with open(f"{cargo_dir}/build.rs", "w") as build_file:
        build_file.write(f"""fn main() {{
    println!("cargo:rustc-link-search={link_path}");
}}""")

    # main.rs, boilerplate.rs
    os.mkdir(f"{cargo_dir}/src")
    os.symlink(os.path.abspath(f"poc/{poc_name}.rs"), f"{cargo_dir}/src/main.rs")
    os.symlink(os.path.abspath(f"poc/boilerplate.rs"), f"{cargo_dir}/src/boilerplate.rs")


def prepare_cargo_cmd(metadata, subcommand):
    result = ["cargo"]

    if "cargo_toolchain" in metadata["test"]:
        result.append("+" + metadata["test"]["cargo_toolchain"])

    result.append(subcommand)

    if "cargo_flags" in metadata["test"]:
        result += metadata["test"]["cargo_flags"]

    return result


# Parse GitHub URL and returns (owner, repository) pair
def parse_repository_url(repository_url):
    if repository_url.endswith(".git"):
        repository_url = repository_url[:-4]
    result = urlparse(repository_url)
    assert result.scheme == "https"
    assert result.netloc == "github.com"
    return result.path.split("/")[1:3]


def cmd_add(args):
    for poc_id_num in range(10000):
        poc_id = str(poc_id_num).rjust(4, '0')
        if poc_id not in poc_id_to_name:
            break

    assert poc_id not in poc_id_to_name
    poc_name = f"{poc_id}-{args.crate}"
    new_poc_file = f"poc/{poc_name}.rs"
    poc_id_to_name[poc_id] = poc_name

    with open(new_poc_file, "w") as f:
        f.write(f"""/*!
```crux-poc
[target]
crate = "{args.crate}"
version = "{args.version}"

[test]
analyzers = []

[report]
title = "issue title"
description = \"\"\"
issue description\"\"\"
code_snippets = []
patched = []
informational = "unsound"
```
!*/
#![forbid(unsafe_code)]

mod boilerplate;

fn main() {{
    boilerplate::init();

    println!("Hello, World!")
}}
""")

    shutil.rmtree("./poc-debug", ignore_errors=True)
    os.mkdir("./poc-debug")
    prepare_cargo_dir(poc_id, "./poc-debug")

    print(f"Successfully Added {poc_name}")


def cmd_run(args):
    poc_id = args.id
    poc_name = poc_id_to_name[poc_id]

    metadata = read_metadata(poc_id)

    with tempfile.TemporaryDirectory() as tmpdir:
        prepare_cargo_dir(poc_id, tmpdir)

        # execute `cargo run` with proper flags
        cmd = prepare_cargo_cmd(metadata, "run")
        subprocess.run(cmd, cwd=tmpdir)

        if args.copy:
            shutil.rmtree("./poc-debug", ignore_errors=True)
            shutil.copytree(tmpdir, "./poc-debug", symlinks=True)


def cmd_report_crate_repo(poc_id, report):
    poc_name = poc_id_to_name[poc_id]
    metadata = read_metadata(poc_id)

    print(f"Reporting {poc_name} to the crate repository")

    # Check if the bug was already reported
    if "issue_date" in metadata["report"]:
        issue_date = metadata["report"]["issue_date"]
        print(f"Already reported on {issue_date}")
        if "issue_url" in metadata["report"]:
            issue_url = metadata["report"]["issue_url"]
            print(f"Issue URL: {issue_url}")
        else:
            print("Issue URL does not exist")
        return

    target_crate = metadata["target"]["crate"]
    crate_metadata = requests.get(f"https://crates.io/api/v1/crates/{target_crate}").json()

    if "repository" in crate_metadata["crate"]:
        repository_url = crate_metadata["crate"]["repository"]
    else:
        repository_url = None

    if repository_url is None:
        print("Repository URL not found in crates.io metadata")
    elif not repository_url.startswith("https://github.com/"):
        print("Automatic reporting is only supported for GitHub")
    else:
        (owner, repo) = parse_repository_url(repository_url)
        print(f"Reporting to {owner}/{repo}")

        # Use GitHub API to report the bug
        url = f"https://api.github.com/repos/{owner}/{repo}/issues"
        result = requests.post(url, headers=GITHUB_CLIENT_HEADERS, json={
            "title": report["title"],
            "body": report["description"],
        })

        if result.status_code == 201:
            result_json = result.json()
            issue_url = result_json["html_url"]
            print(f"Successfully created an issue: {issue_url}")
            append_metadata(poc_id, {
                "issue_url": issue_url,
                "issue_date": datetime.date.today()
            })
        elif result.status_code == 410:
            print("Issue tracker is disabled; Reporting was skipped")
            append_metadata(poc_id, {
                "issue_date": datetime.date.today()
            })
        else:
            print(f"Unknown error {result.status_code}")
            print(result.json())


def cmd_report_rustsec(poc_id, report):
    poc_name = poc_id_to_name[poc_id]
    metadata = read_metadata(poc_id)

    # Prepare report data
    crate_name = metadata["target"]["crate"]
    report_title = metadata["report"]["title"]
    report_description = metadata["report"]["description"]

    informational_str = ""
    if "informational" in metadata["report"]:
        informational_value = metadata["report"]["informational"]
        informational_str = f'informational = "{informational_value}"\n'

    url_value = None
    url_str = ""
    if "issue_url" in metadata["report"]:
        url_value = metadata["report"]["issue_url"]
        url_str = f'url = "{url_value}"\n'

    version_dict = {"patched": metadata["report"]["patched"]}
    if "unaffected" in metadata["report"]:
        version_dict["unaffected"] = metadata["report"]["unaffected"]

    version_str = toml.dumps(version_dict)


    def run_in_advisory_db(args, silent=False):
        if silent:
            return subprocess.run(
                args,
                cwd="advisory-db",
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=True,
            )
        else:
            return subprocess.run(args, cwd="advisory-db", check=True)

    # Start reporting
    print(f"Start creating a commit for {poc_name}")
    branch_name = poc_name

    # `issue_date` field is required for RustSec report
    if not "issue_date" in metadata["report"]:
        print("`issue_date` field does not exist in the metadata")
        return

    issue_date = metadata["report"]["issue_date"]
    crate_dir = f"advisory-db/crates/{crate_name}"
    os.makedirs(crate_dir, exist_ok=True)

    # Update the master branch
    print("Updating master branch...")
    run_in_advisory_db(["git", "checkout", "master"], silent=True)
    run_in_advisory_db(["git", "fetch", "rustsec"], silent=True)
    run_in_advisory_db(["git", "merge", "rustsec/master", "--ff-only"], silent=True)
    run_in_advisory_db(["git", "push"], silent=True)

    print("Switching to a poc branch...")
    try:
        run_in_advisory_db(["git", "checkout", "-b", branch_name], silent=True)
        needs_pr = True
    except subprocess.CalledProcessError:
        # Branch already exists
        run_in_advisory_db(["git", "checkout", branch_name], silent=True)
        needs_pr = False

    with open(f"{crate_dir}/RUSTSEC-0000-0000.toml", "w") as rustsec_toml:
        rustsec_toml.write(f'''[advisory]
id = "RUSTSEC-0000-0000"
package = "{crate_name}"
date = "{issue_date}"
{informational_str}title = "{report_title}"
{url_str}description = """
{report_description}
"""

[versions]
{version_str}''')

    if needs_pr:
        commit_msg = f"Initial report for {poc_name}"
    else:
        commit_msg = f"Update {poc_name}"

    run_in_advisory_db(["git", "add", "-A"], silent=True)
    try:
        run_in_advisory_db(["git", "commit", "-m", commit_msg])
    except subprocess.CalledProcessError:
        print("Empty commit, nothing to update")
        return
    run_in_advisory_db(["git", "push", "-u", "origin", branch_name])

    if needs_pr:
        print(f"Reporting {poc_name} to RustSec")

        if "rustsec_url" in metadata["report"]:
            rustsec_url = metadata["report"]["rustsec_url"]
            print(f"Already reported to: {rustsec_url}")
            return

        if url_value is None:
            report_body = report["description"]
        else:
            report_body = f"""{report_description}

Original issue report: {url_value}"""

        # Use GitHub API to report the bug
        url = f"https://api.github.com/repos/RustSec/advisory-db/pulls"
        result = requests.post(url, headers=GITHUB_CLIENT_HEADERS, json={
            "title": crate_name + ": "+ report["title"],
            "head": f"{user_github_id}:{branch_name}",
            "base": "master",
            "body": report_body,
        })

        if result.status_code == 201:
            result_json = result.json()
            pr_url = result_json["html_url"]
            print(f"Successfully created PR: {pr_url}")
            append_metadata(poc_id, {"rustsec_url": pr_url})
        else:
            print(f"Unknown error {result.status_code}")
            print(result.json())
    else:
        print(f"PR has been updated")


def cmd_report(args):
    if not args.crate_repo and not args.rustsec:
        args.crate_repo = True
        args.rustsec = True

    poc_id = args.id
    report = prepare_report(poc_id)
    if args.preview:
        print(f"Title:\n{report['title']}\n\nDescription:\n{report['description']}")
        return

    if args.crate_repo:
        cmd_report_crate_repo(poc_id, report)

    if args.rustsec:
        cmd_report_rustsec(poc_id, report)


action_dict = {
    "add": cmd_add,
    "run": cmd_run,
    "report": cmd_report,
}

if args.cmd is None:
    parser.print_help()
    sys.exit(1)

action = action_dict[args.cmd]
action(args)

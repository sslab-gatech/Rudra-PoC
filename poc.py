#!/usr/bin/env python

import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile
import toml

from subprocess import PIPE

parser = argparse.ArgumentParser()

subparsers = parser.add_subparsers(dest="cmd")

parser_add = subparsers.add_parser("add")
parser_add.add_argument("crate", help="target crate name")
parser_add.add_argument("version", help="target crate version")

parser_run = subparsers.add_parser("run")
parser_run.add_argument("id", help="poc ID (4 digits)")

args = parser.parse_args()

# maps PoC number to PoC file name
poc_id_to_name = {}

# record known PoC
poc_dir_pattern = re.compile(r"(\d{4})-.+")

for name in os.listdir("poc"):
    if os.path.isfile(f"poc/{name}"):
        match = poc_dir_pattern.match(name)
        poc_id = match.group(1)
        poc_id_to_name[poc_id] = name

# Override rustc to enable the build cache
os.environ["RUSTC_WRAPPER"] = "sccache"


def read_metadata(id):
    poc_id = args.id
    poc_name = poc_id_to_name[poc_id]

    with open(f"poc/{poc_name}") as poc_file:
        lines = list(map(lambda s: s.strip(), poc_file))
        if lines[0] == "/*!" and lines[1] == "```crux-poc":
            idx = lines[2:].index("```") + 2
            toml_str = '\n'.join(lines[2:idx])
            return toml.loads(toml_str)

    return None


def prepare_cargo_cmd(metadata, subcommand):
    result = ["cargo"]

    if "cargo_toolchain" in metadata["test"]:
        result.append("+" + metadata["test"]["cargo_toolchain"])

    result.append(subcommand)

    if "cargo_flags" in metadata["test"]:
        result.append(*metadata["test"]["cargo_flags"])

    return result


def cmd_add(args):
    for poc_id_num in range(10000):
        poc_id_str = str(poc_id_num).rjust(4, '0')
        if poc_id_str not in poc_id_to_name:
            break

    assert poc_id_str not in poc_id_to_name
    new_poc_file = f"poc/{poc_id_str}-{args.crate}.rs"
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
```
!*/

fn main() {{
    println!("Hello, World!")
}}
""")

    print(f"Created `{new_poc_file}` with version {args.version}")


def cmd_run(args):
    poc_id = args.id
    poc_name = poc_id_to_name[poc_id]

    metadata = read_metadata(poc_id)

    with tempfile.TemporaryDirectory() as tmpdir:
        with open(f"{tmpdir}/Cargo.toml", "w") as manifest_file:
            manifest_content = f"""[package]
name = "crux-poc-{poc_id}"
version = "0.1.0"
authors = ["Yechan Bae <yechan@gatech.edu>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
{metadata["target"]["crate"]} = "={metadata["target"]["version"]}"
"""

            if "peer" in metadata["target"]:
                for crate in metadata["target"]["peer"]:
                    manifest_content += f'''{crate["crate"]} = "={crate["version"]}"\n'''

            manifest_file.write(manifest_content)

        os.mkdir(f"{tmpdir}/src")
        shutil.copyfile(f"poc/{poc_name}", f"{tmpdir}/src/main.rs")

        cmd = prepare_cargo_cmd(metadata, "run")

        subprocess.run(cmd, cwd=tmpdir)

# TODO: cmd_report_original
# TODO: cmd_report_rustsec
# TODO: cmd_generate


action_dict = {
    "add": cmd_add,
    "run": cmd_run,
}

if args.cmd is None:
    parser.print_help()
    sys.exit(1)

action = action_dict[args.cmd]
action(args)

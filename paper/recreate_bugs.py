#!/usr/bin/env python3
"""
Usage recreate_bugs.py [optional_crate_name]

Prerequisites:

- `rudra:latest` image exists.
- Have `docker-cargo-rudra` in PATH.
"""
from common import *

import os
import multiprocessing
import pathlib
import re
import shutil
import subprocess
import sys

import tomlkit


ANALYZERS = [
    "SendSyncVariance",
    "UnsafeDataflow",
]


RECREATE_DIRECTORY = PROJECT_DIRECTORY / 'rudra-recreate'
RECREATE_DIRECTORY.mkdir(exist_ok=True)

# Match this with Rudra
TOOLCHAIN_VERSION = "nightly-2020-08-26"

ansi_escape_8bit = re.compile(
    r'(?:\x1B[@-Z\\-_]|[\x80-\x9A\x9C-\x9F]|(?:\x1B\[|\x9B)[0-?]*[ -/]*[@-~])'
)

if "RUDRA_RUNNER_HOME" in os.environ:
    RUNNER_HOME_PATH = pathlib.Path(os.environ["RUDRA_RUNNER_HOME"])
    CARGO_HOME_PATH = RUNNER_HOME_PATH / "cargo_home"
    print("RUDRA_RUNNER_HOME found, frozen index will be used")
else:
    CARGO_HOME_PATH = None
    print("RUDRA_RUNNER_HOME not found, system cargo index will be used")


def process(poc):
    # Skip others if crate name is given
    if len(sys.argv) > 1 and poc['target']['crate'] != sys.argv[1]:
        return

    if 'indexed_name' in poc['target']:
        crate_name = poc['target']['indexed_name']
    else:
        crate_name = poc['target']['crate']

    if 'indexed_version' in poc['target']:
        crate_version = poc['target']['indexed_version']
    else:
        crate_version = poc['target']['version']
    crate_and_version = f"{crate_name}-{crate_version}"
    crate_folder = RECREATE_DIRECTORY / crate_and_version
    crate_report_dir = RECREATE_DIRECTORY / (crate_and_version + "-report")

    # Skip if all bugs were manually found
    if not any(map(lambda bug: bug['analyzer'] != "Manual", poc['bugs'])):
        print(f"Skipping {crate_and_version} - only contains manual bugs")
        return

    # Download the crate if needed.
    if not crate_folder.is_dir():
        print(f"Downloading {crate_and_version}")
        subprocess.check_output(
            ['cargo', 'download', '-x', '-o', str(crate_folder),
            f"{crate_name}=={crate_version}"],
            cwd=RECREATE_DIRECTORY
        )

    # Run rudra inside the folder.
    new_env = os.environ.copy()
    new_env['CARGO_ARGS'] = "--locked -j 1"

    if crate_report_dir.exists():
        shutil.rmtree(crate_report_dir)

    print(f"Start running Rudra for {crate_and_version}")
    result = subprocess.run(
        ['docker-cargo-rudra', str(crate_folder), str(crate_report_dir)],
        env=new_env, capture_output=True
    )
    if result.returncode != 0:
        # Some target in the target crate may fail to build, but we still get report
        print(f'Warning: Some of the targets of {crate_and_version} fail to build')

    count = {}
    for analyzer in ANALYZERS:
        count[analyzer] = {
            "visible": {
                "Error": 0,
                "Warning": 0,
                "Info": 0,
            },
            "internal": {
                "Error": 0,
                "Warning": 0,
                "Info": 0,
            },
        }

    # Populate the bug location set we are aiming to reproduce.
    visible_bug_set = set()
    internal_bug_set = set()
    for bug in poc['bugs']:
        if "reason" in bug:
            # Unreported
            bug_locations = [bug["location"]]
        else:
            # PoC
            bug_locations = bug["rudra_report_locations"]

        for bug_location in bug_locations:
            if "reason" in bug and bug["reason"] == "internal":
                internal_bug_set.add(bug_location)
            else:
                visible_bug_set.add(bug_location)

    # Go over all the reports.
    for report_file in crate_report_dir.glob("*"):
        with open(report_file, 'r') as f:
            report_content = f.read()
            # tomlkit doesn't like the ANSI escape codes in the UD reports.
            # strip them out before parsing.
            report_content = ansi_escape_8bit.sub('', report_content)

            report = tomlkit.parse(report_content)
            for report in report['reports']:
                analyzer = report["analyzer"].split(":")[0]
                level = report["level"]
                
                report_span = report["location"]
                if report_span in visible_bug_set:
                    count[analyzer]["visible"][level] += 1
                    visible_bug_set.remove(report_span)
                elif report_span in internal_bug_set:
                    count[analyzer]["internal"][level] += 1
                    internal_bug_set.remove(report_span)

    if len(visible_bug_set) != 0 or len(internal_bug_set) != 0:
        print('=============================================================')
        print(f'ERROR: Bug locations for {crate_and_version} do not recreate')
        print(f'  Missing: {set.union(visible_bug_set, internal_bug_set)}')
        return

    print(f"Finished running Rudra for {crate_and_version}")
    return count


def main():
    # 3 bugs from std and 2 from rustc not represented in the PoCs
    total_count = {}
    for analyzer in ANALYZERS:
        total_count[analyzer] = {
            "visible": {
                "Error": 1,
                "Warning": 1,
                "Info": 1,
            },
            "internal": {
                "Error": 1,
                "Warning": 1,
                "Info": 0,
            },
        }

    poc_metadata = get_poc_metadata()
    unreported_metadata = get_unreported_metadata()

    with multiprocessing.Pool() as pool:
        count_list = pool.map(process, list(poc_metadata.values()) + list(unreported_metadata.values()))

    for count in count_list:
        if count is not None:
            for analyzer in ANALYZERS:
                for visibility in ["visible", "internal"]:
                    for level in ["Error", "Warning", "Info"]:
                        total_count[analyzer][visibility][level] += count[analyzer][visibility][level]

    for analyzer in ANALYZERS:
        high_visible = total_count[analyzer]["visible"]["Error"]
        high_internal = total_count[analyzer]["internal"]["Error"]
        med_visible = total_count[analyzer]["visible"]["Warning"] + high_visible
        med_internal = total_count[analyzer]["internal"]["Warning"] + high_internal
        low_visible = total_count[analyzer]["visible"]["Info"] + med_visible
        low_internal = total_count[analyzer]["internal"]["Info"] + med_internal

        print(f"Bugs count for {analyzer} algorithm")
        print(f"High - visible {high_visible:3d} / internal {high_internal:3d}")
        print(f" Med - visible {med_visible:3d} / internal {med_internal:3d}")
        print(f" Low - visible {low_visible:3d} / internal {low_internal:3d}")


if __name__ == '__main__':
    main()

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
import subprocess
import sys

import tomlkit


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
    new_env['RUDRA_REPORT_PATH'] = str(crate_folder)
    new_env['RUSTUP_TOOLCHAIN'] = TOOLCHAIN_VERSION
    if CARGO_HOME_PATH is not None:
        new_env['CARGO_HOME'] = str(CARGO_HOME_PATH)
    new_env['CARGO_ARGS'] = "--locked -j 1"

    print(f"Start running Rudra for {crate_and_version}")
    result = subprocess.run(
        ['docker-cargo-rudra', str(crate_folder)],
        cwd=crate_folder, env=new_env, capture_output=True
    )
    if result.returncode != 0:
        print('=============================================================')
        print(f'ERROR: Cargo rudra returned non-zero return value for {crate_and_version}')
        print(f"stdout {crate_and_version}: {result.stdout}")
        print(f"stderr {crate_and_version}: {result.stderr}")
        return

    bug_locations = set()
    # Go over all the reports.
    for report_file in RECREATE_DIRECTORY.glob(crate_and_version + '-*'):
        with open(report_file, 'r') as f:
            report_content = f.read()
            # tomlkit doesn't like the ANSI escape codes in the UD reports.
            # strip them out before parsing.
            report_content = ansi_escape_8bit.sub('', report_content)

            report = tomlkit.parse(report_content)
            for warning in report['reports']:
                bug_locations.add(warning['location'])

    # Assert that all the POC locations are present in the current reports.
    for bug in poc['bugs']:
        for report_location in bug['rudra_report_locations']:
            if report_location not in bug_locations:
                print('=============================================================')
                print(f'ERROR: Bug locations for {crate_and_version} do not recreate')
                print(f'  Bug locations from cargo-rudra: {bug_locations}')
                print(f'  Missing: {report_location}')
                return

    print(f"Finished running Rudra for {crate_and_version}")


def main():
    poc_metadata = get_poc_metadata()

    with multiprocessing.Pool() as pool:
        pool.map(process, poc_metadata.values())


if __name__ == '__main__':
    main()

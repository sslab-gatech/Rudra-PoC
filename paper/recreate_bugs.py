#!/usr/bin/env python3
"""
Usage recreate_bugs.py [optional_crate_name]

Run with 2>/dev/null for less noisy output.

Prerequisites:

- `cargo install cargo-download`
- Have `cargo-rudra` in PATH.
"""
from common import *
import re
import subprocess
import sys
import os

import tomlkit


RECREATE_DIRECTORY = PROJECT_DIRECTORY / 'rudra-recreate'
ansi_escape_8bit = re.compile(
    r'(?:\x1B[@-Z\\-_]|[\x80-\x9A\x9C-\x9F]|(?:\x1B\[|\x9B)[0-?]*[ -/]*[@-~])'
)

# Needs debugging, these crates don't compile as of now, might be a bump of
# their dependencies that violates semvers. Maybe we should try to pin them
# to analysis time installations.
#
#slock:
#   error[E0635]: unknown feature `const_type_id`
#   --> /registry/src/github.com-1ecc6299db9ec823/value-bag-1.0.0-alpha.6/src/lib.rs:7:54
#   |
#   7 | #![cfg_attr(value_bag_capture_const_type_id, feature(const_type_id))]
# 
#containers:
# no method named `assume_init_ref` found for union `core::mem::MaybeUninit<(K, T)>` in the current scope
#    --> /registry/src/github.com-1ecc6299db9ec823/hash-table-0.2.5/src/lib.rs:157:50
#     |
# 157 |             let (_, x) = ptr::read(self.elems[i].assume_init_ref());
#     |                                                  ^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `assume_init`
NON_COMPILING_CRATES = {'containers', 'slock'}


# TODO: compare `bug_count` and `rudra_report_locations` for non-manual bugs
def main():
    poc_metadata = get_poc_metadata()

    for poc in poc_metadata.values():
        if len(sys.argv) > 1 and poc['target']['crate'] != sys.argv[1]:
            continue

        if poc['target']['crate'] in NON_COMPILING_CRATES:
            print('WARNING: Skipping crate.')
            continue

        # Download the crate if needed.
        crate_and_version = f"{poc['target']['crate']}-{poc['target']['version']}"
        crate_folder = RECREATE_DIRECTORY / crate_and_version

        if not crate_folder.is_dir():
            print(f"Downloading {crate_and_version}")
            subprocess.check_output(
                ['cargo', 'download', '-x', '-o', str(crate_folder),
                f"{poc['target']['crate']}=={poc['target']['version']}"],
                cwd=RECREATE_DIRECTORY
            )

        # Run rudra inside the folder.
        new_env = os.environ.copy()
        new_env['RUDRA_REPORT_PATH'] = str(crate_folder)

        print(f"Running Rudra for {crate_and_version}")
        rudra_output = subprocess.check_call(
            ['cargo', 'rudra'], cwd=crate_folder, env=new_env
        )

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
                    print(f'Bug locations from cargo-rudra: {bug_locations}')
                    print(f'Missing: {report_location}')
                    sys.exit(1)


if __name__ == '__main__':
    main()

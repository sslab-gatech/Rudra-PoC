#!/usr/bin/env python3
from common import *

import csv
import os
import pathlib

import semver

class Crate:
    def __init__(self, dict):
        self.id = int(dict["id"])
        self.name = dict["name"]

class Version:
    def __init__(self, dict):
        self.crate_id = int(dict["crate_id"])
        self.num = semver.VersionInfo.parse(dict["num"])

RUNNER_HOME_PATH = pathlib.Path(os.environ["RUDRA_RUNNER_HOME"])
DB_DUMP_PATH = RUNNER_HOME_PATH / "rudra_cache" / "db-dump" / "2020-07-04-140112"
CRATES_CSV_PATH = DB_DUMP_PATH / "data" / "crates.csv"
VERSIONS_CSV_PATH = DB_DUMP_PATH / "data" / "versions.csv"

def main():
    rustsec_metadata = get_rustsec_metadata()
    poc_metadata = get_poc_metadata()

    print("=======================")
    print("Checking crate versions")
    print("=======================")

    name_to_id = {}
    id_to_name = {}
    id_to_versions = {}

    csv.field_size_limit(524288)

    with open(CRATES_CSV_PATH, newline='') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            crate = Crate(row)
            id_to_name[crate.id] = crate.name
            name_to_id[crate.name] = crate.id

    with open(VERSIONS_CSV_PATH, newline='') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            try:
                version = Version(row)
            except ValueError as e:
                print(f"WARNING: {e.args[0]}")
            if version.crate_id not in id_to_versions:
                id_to_versions[version.crate_id] = []
            id_to_versions[version.crate_id].append(version.num)

    for (poc_id, poc) in poc_metadata.items():
        if 'indexed_name' in poc['target']:
            crate_name = poc['target']['indexed_name']
        else:
            crate_name = poc['target']['crate']
        if 'indexed_version' in poc['target']:
            crate_version = semver.VersionInfo.parse(poc['target']['indexed_version'])
        else:
            crate_version = semver.VersionInfo.parse(poc['target']['version'])

        crate_id = name_to_id[crate_name]
        if crate_version not in id_to_versions[crate_id]:
            print(f"PoC {poc_id} - {crate_name}: {crate_version} not found")
            print("  [{}]".format(", ".join(map(str, id_to_versions[crate_id]))))
            print("  latest: {}".format(str(max(id_to_versions[crate_id]))))
            print()

    print("===================")
    print("Checking bug counts")
    print("===================")

    for (poc_id, poc) in poc_metadata.items():
        crate_name = poc['target']['crate']

        for bug in poc['bugs']:
            if 'bug_count' in bug:
                bug_count = int(bug['bug_count'])
            else:
                bug_count = 1
            bug_locations = len(bug['rudra_report_locations'])
            if bug['analyzer'] != 'Manual':
                if bug_count != bug_locations:
                    print(f"PoC {poc_id} - {crate_name} bug count mismatch")
                    print(f"  bug count: {bug_count}")
                    print(f"  bug locations: {bug_locations}")
                    print()
            else:
                if bug_locations > 0:
                    print(f"PoC {poc_id} - {crate_name} contains bug locations for manual bugs")
                    print()


if __name__ == "__main__":
    main()
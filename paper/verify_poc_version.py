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
        crate_name = poc['target']['crate']
        crate_version = semver.VersionInfo.parse(poc['target']['version'])

        crate_id = name_to_id[crate_name]
        if crate_version not in id_to_versions[crate_id]:
            print(f"PoC {poc_id} - {crate_name}: {crate_version} not found")
            print("  [{}]".format(", ".join(map(str, id_to_versions[crate_id]))))

if __name__ == "__main__":
    main()
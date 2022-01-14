#!/usr/bin/env python3
from common import *
from collections import defaultdict
import sys


def main():
    cve_data = []

    rustsec_metadata_dict = get_rustsec_metadata()
    poc_metadata_dict = get_poc_metadata()

    for (poc_id, poc_metadata) in poc_metadata_dict.items():
        if 'issue_date' not in poc_metadata['report']:
            print(f"Warning: PoC {poc_id} is not reported")
            continue

        crate_name = poc_metadata['target']['crate']

        issue_date = poc_metadata['report']['issue_date']
        issue_year = issue_date.year
        issue_date_string = f"{issue_date.year}/{issue_date.month:02}/{issue_date.day:02}"

        try:
            rustsec_id = poc_metadata['report']['rustsec_id']
        except tomlkit.exceptions.NonExistentKey:
            # Pending bugs
            continue

        rustsec_metadata = rustsec_metadata_dict[rustsec_id]
        rustsec_title = rustsec_metadata["title"]

        cve_list = []
        if 'aliases' in rustsec_metadata:
            for alias in rustsec_metadata['aliases']:
                if alias.startswith("CVE"):
                    cve_list.append(alias)

        if len(cve_list) > 0:
            cve_text = ", ".join(cve_list)
            print(f"""date: {issue_date_string}
proj: {crate_name} (Rust)
cve: {cve_text}
desc: {rustsec_title}
url: https://rustsec.org/advisories/{rustsec_id}.html
lead: Rudra project members
""")

if __name__ == '__main__':
    main()

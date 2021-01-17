#!/usr/bin/env python3
from common import *

def main():
    rustsec_metadata = get_rustsec_metadata()
    poc_metadata = get_poc_metadata()

    cve_cnt = 0
    reported_cnt = 0
    backlog_cnt = 0

    reported_by_year = {}
    backlog_by_year = {}

    ours_id_set = set()
    for poc_metadata in poc_metadata.values():
        issue_date = poc_metadata['report']['issue_date']
        issue_year = issue_date.year
        try:
            ours_id_set.add(poc_metadata['report']['rustsec_id'])

            if issue_year not in reported_by_year:
                reported_by_year[issue_year] = 0
            reported_by_year[issue_year] += 1
            reported_cnt += 1
        except tomlkit.exceptions.NonExistentKey:
            if issue_year not in backlog_by_year:
                backlog_by_year[issue_year] = 0
            backlog_by_year[issue_year] += 1
            backlog_cnt += 1

    for (bug_id, rustsec_metadata) in sorted(rustsec_metadata.items()):
        ours = bug_id in ours_id_set
        if ours:
            cve_found = False
            if 'aliases' in rustsec_metadata:
                for alias in rustsec_metadata['aliases']:
                    if alias.startswith("CVE"):
                        cve_found = True
                        cve_cnt += 1
            if not cve_found:
                package = rustsec_metadata['package']
                print(f"Bug {bug_id} ({package}) does not have a CVE number!")

    print(f"Total CVE: {cve_cnt}")

    print(f"Reported: {reported_cnt}")
    for (year, count) in reported_by_year.items():
        print(f"{year}: {count}")

    print(f"Not yet reported: {backlog_cnt}")
    for (year, count) in backlog_by_year.items():
        print(f"{year}: {count}")

if __name__ == '__main__':
    main()

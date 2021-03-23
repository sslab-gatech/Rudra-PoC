#!/usr/bin/env python3
import csv

from common import *

def main():
    rustsec_metadata = get_rustsec_metadata()
    poc_metadata = get_poc_metadata()

    backlog_by_year = {}
    ours_id_set = set()
    for poc_metadata in poc_metadata.values():
        if 'issue_date' not in poc_metadata['report']:
            continue

        if 'rustsec_id' in poc_metadata['report']:
            # Put non-manual bugs into ID set
            if any(map(lambda bug: bug['analyzer'] != "Manual", poc_metadata['bugs'])):
                ours_id_set.add(poc_metadata['report']['rustsec_id'])
        else:
            # Count non-reported bugs
            issue_date = poc_metadata['report']['issue_date']
            issue_year = issue_date.year
            if issue_year not in backlog_by_year:
                backlog_by_year[issue_year] = 0
            backlog_by_year[issue_year] += 1

    with open("rustsec_list.csv", "w", newline="") as csvfile:
        COLUMN_NAMES = ["id", "year", "type", "ours"]
        csv_writer = csv.writer(csvfile)
        csv_writer.writerow(COLUMN_NAMES)
    
        for (bug_id, rustsec_metadata) in sorted(rustsec_metadata.items()):
            if bug_id.startswith("CVE-"):
                year = bug_id[4:8]
            elif bug_id.startswith("RUSTSEC-"):
                year = bug_id[8:12]

            if "informational" in rustsec_metadata:
                if rustsec_metadata["informational"] == "unmaintained":
                    bug_type = "unmaintained"
                elif rustsec_metadata["informational"] == "unsound":
                    bug_type = ""
                else:
                    bug_type = "unknown"
            else:
                bug_type = ""

            ours = bug_id in ours_id_set
            csv_writer.writerow([bug_id, year, bug_type, ours])

    print("Not yet reported")
    for (year, count) in backlog_by_year.items():
        print(f"{year}: {count}")

if __name__ == '__main__':
    main()

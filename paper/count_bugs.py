#!/usr/bin/env python3
from common import *

def main():
    rustsec_metadata = get_rustsec_metadata()
    poc_metadata = get_poc_metadata()

    cve_crates = 0
    cve_cnt = 0

    reported_cnt = 0
    backlog_cnt = 0

    # 2 bugs from std not represented in the PoCs
    total_bugs_cnt = 2

    send_sync_variance_crates = 0
    send_sync_variance_cnt = 0
    unsafe_dataflow_crates = 0
    unsafe_dataflow_cnt = 0

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

        num_bugs_in_poc = poc_metadata['report']['unique_bugs']
        if 'additional_send_sync_violations' in poc_metadata['report']:
            num_bugs_in_poc += poc_metadata['report']['additional_send_sync_violations']

        total_bugs_cnt += num_bugs_in_poc

        both = False

        if 'SendSyncVariance' in poc_metadata['test']['analyzers']:
            send_sync_variance_crates += 1
            send_sync_variance_cnt += num_bugs_in_poc
            assert both == False
            both = True

        if 'UnsafeDataflow' in poc_metadata['test']['analyzers']:
            unsafe_dataflow_crates += 1
            unsafe_dataflow_cnt += num_bugs_in_poc
            assert both == False
            both = True

    for (bug_id, rustsec_metadata) in sorted(rustsec_metadata.items()):
        ours = bug_id in ours_id_set
        if ours:
            cve_found = False
            if 'aliases' in rustsec_metadata:
                for alias in rustsec_metadata['aliases']:
                    if alias.startswith("CVE"):
                        cve_found = True
                        cve_cnt += 1
            if cve_found:
                cve_crates += 1
            else:
                # package = rustsec_metadata['package']
                # print(f"Bug {bug_id} ({package}) does not have a CVE number!")
                pass

    print(f"Not yet reported PoC: {backlog_cnt}")
    for (year, count) in backlog_by_year.items():
        print(f"  {year}: {count}")

    print(f"Reported PoC: {reported_cnt}")
    for (year, count) in reported_by_year.items():
        print(f"  {year}: {count}")

    print(f"Among {reported_cnt} RustSec advisories, {cve_crates} advisories received {cve_cnt} CVEs")
    print(f"PoCs contain total of {total_bugs_cnt} bugs")

    print("Unsafe Dataflow")
    print(f"  Crates: {unsafe_dataflow_crates} / Bugs: {unsafe_dataflow_cnt}")

    print(f"Send Sync Variance")
    print(f"  Crates: {send_sync_variance_crates} / Bugs: {send_sync_variance_cnt}")

if __name__ == '__main__':
    main()

#!/usr/bin/env python3
from common import *

def main():
    rustsec_metadata = get_rustsec_metadata()
    poc_metadata = get_poc_metadata()

    cve_crates = 0
    cve_cnt = 0

    reported_cnt = 0
    backlog_cnt = 0

    # 2 bugs from std and 1 from rustc not represented in the PoCs
    analyzer_bug_cnt = 3
    manual_bug_cnt = 0

    analyzer_crate_set = {"std"}
    manual_crate_set = set()

    # TODO: add strict count - those without "guide"
    send_sync_variance_crate_set = {"std"}
    send_sync_variance_cnt = 1
    unsafe_dataflow_crate_set = {"std"}
    unsafe_dataflow_cnt = 2

    # TODO: Add three std/rustc bugs to year set
    reported_by_year = {}
    backlog_by_year = {}

    ours_id_set = set()
    for (poc_id, poc_metadata) in poc_metadata.items():
        if 'issue_date' not in poc_metadata['report']:
            print(f"Warning: PoC {poc_id} is not reported")
            continue

        crate_name = poc_metadata['target']['crate']

        issue_date = poc_metadata['report']['issue_date']
        issue_year = issue_date.year
        try:
            # TODO: exclude manually found bugs
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

        if any(map(lambda bug: bug['analyzer'] == 'SendSyncVariance', poc_metadata['bugs'])):
            send_sync_variance_crate_set.add(crate_name)

        if any(map(lambda bug: bug['analyzer'] == 'UnsafeDataflow', poc_metadata['bugs'])):
            unsafe_dataflow_crate_set.add(crate_name)

        for bug in poc_metadata['bugs']:
            # Default bug count is 1
            if 'bug_count' in bug:
                bug_count = bug['bug_count']
            else:
                bug_count = 1

            if bug['analyzer'] == 'SendSyncVariance':
                send_sync_variance_cnt += bug_count
            elif bug['analyzer'] == 'UnsafeDataflow':
                unsafe_dataflow_cnt += bug_count

            if bug['analyzer'] == 'Manual':
                manual_bug_cnt += bug_count
                manual_crate_set.add(crate_name)
            else:
                analyzer_bug_cnt += bug_count
                analyzer_crate_set.add(crate_name)

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
    print(f"Total of {analyzer_bug_cnt} bugs in {len(analyzer_crate_set)} crates")
    print(f"Additional {manual_bug_cnt} bugs in {len(manual_crate_set)} crates that are manually found")

    print("UnsafeDataflow")
    print(f"  Crates: {len(send_sync_variance_crate_set)} / Bugs: {unsafe_dataflow_cnt}")

    print(f"SendSyncVariance")
    print(f"  Crates: {len(unsafe_dataflow_crate_set)} / Bugs: {send_sync_variance_cnt}")

    print("""
Paste this in cmds.tex:
\\newcommand{\\bugcount}{%d\\xspace}
\\newcommand{\\buggycratecount}{%d\\xspace}
\\newcommand{\\rustseccount}{%d\\xspace}
\\newcommand{\\cvecount}{%d\\xspace}

\\newcommand{\\udbugcount}{%d\\xspace}
\\newcommand{\\svbugcount}{%d\\xspace}""" % (
        analyzer_bug_cnt,
        len(analyzer_crate_set),
        reported_cnt,
        cve_cnt,
        unsafe_dataflow_cnt,
        send_sync_variance_cnt,
    ))

if __name__ == '__main__':
    main()

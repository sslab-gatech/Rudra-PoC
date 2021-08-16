#!/usr/bin/env python3
from common import *
from collections import defaultdict
import sys


class BugCounter:
    def __init__(self):
        self.cve_count = 0
        self.rustsec_id_set = set()
        self.crate_set = set()
        # bug class -> count
        self.bugs = defaultdict(int)


    def bug_count(self):
        return sum(self.bugs.values())


    def __str__(self):
        crate_count = len(self.crate_set)
        bugs_count = self.bug_count()
        rustsec_count = len(self.rustsec_id_set)
        return f"Crate: {crate_count}, Bugs: {bugs_count}, RustSec: {rustsec_count}, CVE: {self.cve_count}"


def main():
    simple = len(sys.argv) > 1 and sys.argv[1] == "--simple"

    rustsec_metadata = get_rustsec_metadata()
    poc_metadata = get_poc_metadata()

    # 2 bugs from std and 1 from rustc not represented in the PoCs
    # https://github.com/rust-lang/rust/issues/80894
    # https://github.com/rust-lang/rust/issues/80335
    # https://github.com/rust-lang/rust/issues/81425
    rustsec_reported = 3
    rustsec_backlog = 0

    reported_by_year = defaultdict(int)
    reported_by_year[2020] = 1
    reported_by_year[2021] = 2
    backlog_by_year = defaultdict(int)

    total_pending_bugs = 0
    pending_bugs_by_year = defaultdict(int)

    manual = BugCounter()
    send_sync = BugCounter()
    unsafe_dataflow = BugCounter()

    send_sync.crate_set.add("rustc")
    send_sync.bugs["SendSyncVariance"] = 1

    unsafe_dataflow.crate_set.add("std")
    unsafe_dataflow.bugs["HigherOrderInvariant"] = 2

    for (poc_id, poc_metadata) in poc_metadata.items():
        if 'issue_date' not in poc_metadata['report']:
            if not simple:
                print(f"Warning: PoC {poc_id} is not reported")
            continue

        crate_name = poc_metadata['target']['crate']

        contains_sv_bug = any(map(lambda bug: bug['analyzer'] == 'SendSyncVariance', poc_metadata['bugs']))
        contains_ud_bug = any(map(lambda bug: bug['analyzer'] == 'UnsafeDataflow', poc_metadata['bugs']))

        issue_date = poc_metadata['report']['issue_date']
        issue_year = issue_date.year

        if contains_sv_bug and contains_ud_bug:
            print(f"Error: PoC {poc_id} contains both SV and UD bugs")
            exit(1)
        elif contains_sv_bug:
            send_sync.crate_set.add(crate_name)
        elif contains_ud_bug:
            unsafe_dataflow.crate_set.add(crate_name)
        else:
            manual.crate_set.add(crate_name)

        is_pending = False
        non_manual_exists = contains_sv_bug or contains_ud_bug
        if non_manual_exists:
            try:
                rustsec_id = poc_metadata['report']['rustsec_id']

                if contains_sv_bug:
                    send_sync.rustsec_id_set.add(rustsec_id)
                elif contains_ud_bug:
                    unsafe_dataflow.rustsec_id_set.add(rustsec_id)

                reported_by_year[issue_year] += 1
                rustsec_reported += 1
            except tomlkit.exceptions.NonExistentKey:
                backlog_by_year[issue_year] += 1
                rustsec_backlog += 1
                is_pending = True
        else:
            try:
                manual.rustsec_id_set.add(poc_metadata['report']['rustsec_id'])
            except tomlkit.exceptions.NonExistentKey:
                pass

        for bug in poc_metadata['bugs']:
            # Default bug count is 1
            if 'bug_count' in bug:
                bug_count = bug['bug_count']
            else:
                bug_count = 1

            analyzer_name = bug['analyzer']
            bug_class = bug['bug_class']
            if analyzer_name == 'Manual':
                manual.bugs[bug_class] += bug_count
            else:
                if is_pending:
                    total_pending_bugs += bug_count
                    pending_bugs_by_year[issue_year] += bug_count

                if analyzer_name == 'SendSyncVariance':
                    send_sync.bugs[bug_class] += bug_count
                elif analyzer_name == 'UnsafeDataflow':
                    unsafe_dataflow.bugs[bug_class] += bug_count
                else:
                    print(f"Error: Unknown analyzer {analyzer_name} in PoC {poc_id}")
                    exit(1)

    for (bug_id, rustsec_metadata) in sorted(rustsec_metadata.items()):
        if 'aliases' in rustsec_metadata:
            for alias in rustsec_metadata['aliases']:
                if alias.startswith("CVE"):
                    if bug_id in send_sync.rustsec_id_set:
                        send_sync.cve_count += 1
                    elif bug_id in unsafe_dataflow.rustsec_id_set:
                        unsafe_dataflow.cve_count += 1
                    elif bug_id in manual.rustsec_id_set:
                        manual.cve_count += 1


    print("UnsafeDataflow")
    print("  " + str(unsafe_dataflow))
    
    print("SendSyncVariance")
    print("  " + str(send_sync))
    
    print("Manual")
    print("  " + str(manual))

    if simple:
        return


    # Some bug reports are pending because there is no fix available for them,
    # i.e., they are waiting for either the maintainer’s fix
    # or an RFC in the standard library.
    print(f"Not yet reported PoC: {rustsec_backlog}")
    for (year, count) in backlog_by_year.items():
        print(f"  {year}: {count}")

    print(f"Reported PoC: {rustsec_reported}")
    for (year, count) in reported_by_year.items():
        print(f"  {year}: {count}")


    print("""
Paste this in `data/count_bugs.tex`:
\\newcommand{\\newBugCount}{%d\\xspace}
\\newcommand{\\buggyCrateCount}{%d\\xspace}
\\newcommand{\\rustsecCount}{%d\\xspace}
\\newcommand{\\cveCount}{%d\\xspace}

\\newcommand{\\udCrateCount}{%d\\xspace}
\\newcommand{\\udNewBugCount}{%d\\xspace}
\\newcommand{\\udRustsecCount}{%d\\xspace}
\\newcommand{\\udCveCount}{%d\\xspace}

\\newcommand{\\svCrateCount}{%d\\xspace}
\\newcommand{\\svNewBugCount}{%d\\xspace}
\\newcommand{\\svRustsecCount}{%d\\xspace}
\\newcommand{\\svCveCount}{%d\\xspace}

\\newcommand{\\manualCrateCount}{%d\\xspace}
\\newcommand{\\manualNewBugCount}{%d\\xspace}
\\newcommand{\\manualRustsecCount}{%d\\xspace}
\\newcommand{\\manualCveCount}{%d\\xspace}

\\newcommand{\\udHigherorderCount}{%d\\xspace}
\\newcommand{\\udUninitCount}{%d\\xspace}
\\newcommand{\\udPanicSafetyCount}{%d\\xspace}
\\newcommand{\\udOtherCount}{%d\\xspace}

\\newcommand{\\rustsecCountTwenty}{%d\\xspace}
\\newcommand{\\rustsecCountTwentyOne}{%d\\xspace}

\\newcommand{\\pocPendingTwenty}{%d\\xspace}
\\newcommand{\\pocPendingTwentyOne}{%d\\xspace}
\\newcommand{\\pocPendingTotal}{%d\\xspace}

\\newcommand{\\bugPendingTwenty}{%d\\xspace}
\\newcommand{\\bugPendingTwentyOne}{%d\\xspace}
\\newcommand{\\bugPendingTotal}{%d\\xspace}
""" % (
        unsafe_dataflow.bug_count() + send_sync.bug_count(),
        len(unsafe_dataflow.crate_set.union(send_sync.crate_set)),
        len(unsafe_dataflow.rustsec_id_set) + len(send_sync.rustsec_id_set),
        unsafe_dataflow.cve_count + send_sync.cve_count,
        #
        len(unsafe_dataflow.crate_set),
        unsafe_dataflow.bug_count(),
        len(unsafe_dataflow.rustsec_id_set),
        unsafe_dataflow.cve_count,
        #
        len(send_sync.crate_set),
        send_sync.bug_count(),
        len(send_sync.rustsec_id_set),
        send_sync.cve_count,
        #
        len(manual.crate_set),
        manual.bug_count(),
        len(manual.rustsec_id_set),
        manual.cve_count,
        #
        unsafe_dataflow.bugs["HigherOrderInvariant"],
        unsafe_dataflow.bugs["UninitExposure"],
        unsafe_dataflow.bugs["PanicSafety"],
        unsafe_dataflow.bugs["Other"],
        #
        reported_by_year[2020],
        reported_by_year[2021],
        #
        backlog_by_year[2020],
        backlog_by_year[2021],
        rustsec_backlog,
        #
        pending_bugs_by_year[2020],
        pending_bugs_by_year[2021],
        total_pending_bugs,
    ))

if __name__ == '__main__':
    main()

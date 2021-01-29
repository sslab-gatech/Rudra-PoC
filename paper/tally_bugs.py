#!/usr/bin/env python3
from common import *

import collections

def main():
    poc_metadata = get_poc_metadata()

    bug_count = 0
    send_sync_violations = 0
    bugs_per_pattern = collections.Counter()

    # 2 bugs from std and 1 from rustc not represented in the PoCs.
    bug_count += 3

    unreported_bug_count = 0

    for poc_data in poc_metadata.values():
        bug_count += poc_data['report']['unique_bugs']
        print(bug_count)

        for bug_class in poc_data['test']['bug_classes']:
            bugs_per_pattern[bug_class] += poc_data['report']['unique_bugs']

        if 'additional_send_sync_violations' in poc_data['report']:
            send_sync_violations += poc_data['report']['additional_send_sync_violations']

        if 'rustsec_id' not in poc_data['report']:
            unreported_bug_count += poc_data['report']['unique_bugs']
        
    print(f'Total Bugs: {bug_count}')
    print(f'Total Bugs (with each send/sync violation): {bug_count + send_sync_violations}')
    print(f'Total Bugs unreported: {unreported_bug_count}')
    print(bugs_per_pattern)

if __name__ == '__main__':
    main()

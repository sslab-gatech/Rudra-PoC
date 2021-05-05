#!/usr/bin/env python3
from common import *
from collections import defaultdict
from enum import IntEnum
import glob
import os
import sys

# When counting bugs per rank,
# incorporate higher rank counts to lower rank counts
class Rank(IntEnum):
    HIGH = 2
    MID = 1
    LOW = 0

HASHER = {
    ## SendSyncVariance
    "NaiveSendForSend": Rank.LOW,
    "NaiveSyncForSync": Rank.LOW,
    #
    "ApiSyncforSync": Rank.MID,
    "PhantomSendForSend": Rank.MID,
    "RelaxSync": Rank.MID,
    #
    "ApiSendForSync": Rank.HIGH,
    "RelaxSend": Rank.HIGH,
    
    ## UnsafeDataflow
    "PtrAsRef": Rank.LOW,
    "SliceFromRaw": Rank.LOW,
    "SliceUnchecked": Rank.LOW,
    "Transmute": Rank.LOW,
    #
    "CopyFlow": Rank.MID,
    "ReadFlow": Rank.MID,
    "WriteFlow": Rank.MID,
    #
    "VecFromRaw": Rank.HIGH,
    "VecSetLen": Rank.HIGH,
}

def create_counter() -> dict:
    cnt = {
        "SendSyncVariance": {
            Rank.HIGH: 0,
            Rank.MID: 0,
            Rank.LOW: 0,
        },
        "UnsafeDataflow": {
            Rank.HIGH: 0,
            Rank.MID: 0,
            Rank.LOW: 0,
        },
    }
    return cnt

def count_reported_tp(
    poc_detailed_metadata,
    report_dir
) -> dict:
    assert (os.path.isdir(report_dir)), "invalid report directory"

    # tp cnt
    cnt = create_counter()

    analyzers = ['SendSyncVariance', 'UnsafeDataflow']
    for analyzer in analyzers:
        for crate_id, metadata in poc_detailed_metadata[analyzer].items():
            report_file_paths = list(glob.glob(f"{report_dir}/report-{crate_id}-*"))

            assert len(report_file_paths) == 1 , f"{crate_id} {len(report_file_paths)}"

            for report_file_path in report_file_paths:
                with open(report_file_path, "r") as report_file:
                    # match this with Rudra implementation
                    report_file_str = report_file.read().replace("\t", "\\t").replace("\u001B", "\\u001B")
                    report_dict = tomlkit.loads(report_file_str)

                    # Used to get rid of duplicate reports for the same bug-type & span (macro deduplication)
                    visited = dict()
                    for report in filter(lambda x: x['analyzer'].startswith(analyzer), report_dict["reports"]):
                        if report['location'] in metadata:
                            analyzer_subcategories = report['analyzer'][len(analyzer) + 2:].split('/')
                            for subcategory in analyzer_subcategories:
                                key = report['location']
                                if key not in visited or visited[key] < HASHER[subcategory]:
                                    visited[key] = HASHER[subcategory]

                    # Update TP counts
                    for loc, highest_reached_rank in visited.items():
                        for rank in Rank:
                            if highest_reached_rank >= rank:
                                cnt[analyzer][rank] += 1

    return cnt

def count_unreported_tp(report_dir):
    assert (os.path.isdir(report_dir)), "invalid report directory"

    cnt = {
        "internal": create_counter(),
        "other": create_counter(),
    }

    unreported_metadata = get_unreported_metadata()

    analyzers = ['SendSyncVariance', 'UnsafeDataflow']
    for analyzer in analyzers:
        for crate_id, metadata in unreported_metadata[analyzer].items():
            report_file_paths = list(glob.glob(f"{report_dir}/report-{crate_id}-*"))
            assert len(report_file_paths) >= 1, crate_id

            for report_file_path in report_file_paths:
                with open(report_file_path, "r") as report_file:
                    # match this with Rudra implementation
                    report_file_str = report_file.read().replace("\t", "\\t").replace("\u001B", "\\u001B")
                    report_dict = tomlkit.loads(report_file_str)

                    # Used to get rid of duplicate reports for the same bug-type & span (macro deduplication)
                    visited = dict()

                    for report in filter(lambda x: x['analyzer'].startswith(analyzer), report_dict['reports']):
                        if report['location'] in metadata:
                            analyzer_subcategories = report['analyzer'][len(analyzer)+2:].split('/')
                            for subcategory in analyzer_subcategories:
                                key = report['location']
                                if key not in visited or visited[key][0] < HASHER[subcategory]:
                                    is_internal = metadata[report['location']]
                                    visited[key] = (HASHER[subcategory], is_internal)
                    # Update TP counts
                    for (highest_reached_rank, is_internal) in visited.values():
                        for rank in Rank:
                            if highest_reached_rank >= rank:
                                cnt['internal' if is_internal else "other"][analyzer][rank] += 1
    return cnt

if __name__ == "__main__":
    detailed_poc_metadata = get_poc_detailed_metadata()

    # Dictionary containing TruePositive counts for reported bugs
    print('TP (Reported) Count... (by rank)')
    tp_cnt = count_reported_tp(
        detailed_poc_metadata,
        f'/home/youngsuk/rudra-runner-home/campaign/{sys.argv[1]}/report'
    )
    print(tp_cnt)
    
    unreported_tp_cnt = count_unreported_tp(f'/home/youngsuk/rudra-runner-home/campaign/{sys.argv[1]}/report')
    print('TP (Unreported) Count (by rank)')
    print(unreported_tp_cnt)

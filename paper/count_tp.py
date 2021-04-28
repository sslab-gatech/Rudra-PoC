#!/usr/bin/env python3
from common import *
from collections import defaultdict
import glob
import os

NeedSend = {
    "ApiSendForSync",
    "PhantomSendForSend",
    "NaiveSendForSend",
    "RelaxSend",
}
NeedSync = {
    "ApiSyncforSync",
    "NaiveSyncForSync",
    "RelaxSync",
}

def create_counter() -> dict:
    cnt = {
        "SendSyncVariance": {
            "ApiSendForSync": 0,
            "ApiSyncforSync": 0,
            "PhantomSendForSend": 0,
            "NaiveSendForSend": 0,
            "NaiveSyncForSync": 0,
            "RelaxSend": 0,
            "RelaxSync": 0
        },
        "UnsafeDataflow": {
            "ReadFlow": 0,
            "CopyFlow": 0,
            "VecFromRaw": 0,
            "Transmute": 0,
            "WriteFlow": 0,
            "PtrAsRef": 0,
            "SliceUnchecked": 0,
            "SliceFromRaw": 0
        },
    }
    return cnt


def count_reported_tp(
    poc_detailed_metadata,
    report_dir
) -> dict:
    assert (os.path.isdir(report_dir)), "invalid report directory"

    cnt = create_counter()

    # SendSyncVariance #
    for crate_id, metadata in poc_detailed_metadata['SendSyncVariance'].items():
        report_file_paths = list(glob.glob(f"{report_dir}/report-{crate_id}-*"))

        assert len(report_file_paths) == 1 or crate_id in version_discrepancies, f"{crate_id} {len(report_file_paths)}"

        for report_file_path in report_file_paths:
            with open(report_file_path, "r") as report_file:
                # match this with Rudra implementation
                report_file_str = report_file.read().replace("\t", "\\t").replace("\u001B", "\\u001B")
                report_dict = tomlkit.loads(report_file_str)

                # Used to get rid of duplicate reports for the same bug-type & span (macro deduplication)
                visited = set()

                # Filter SendSyncVariance reports
                for report in filter(lambda x: x['analyzer'].startswith('SendSyncVariance'), report_dict["reports"]):
                    if report['location'] in metadata:
                        analyzer_subcategories = report['analyzer'][18:].split('/')
                        for subcategory in analyzer_subcategories:
                            key = (report['location'], subcategory)
                            if key not in visited:
                                cnt['SendSyncVariance'][subcategory] += 1
                                visited.add(key)

    # UnsafeDataflow #
    for crate_id, metadata in poc_detailed_metadata['UnsafeDataflow'].items():
        report_file_paths = list(glob.glob(f"{report_dir}/report-{crate_id}-*"))

        assert len(report_file_paths) == 1 , crate_id

        for report_file_path in report_file_paths:
            with open(report_file_path, "r") as report_file:
                # match this with Rudra implementation
                report_file_str = report_file.read().replace("\t", "\\t").replace("\u001B", "\\u001B")
                report_dict = tomlkit.loads(report_file_str) 

                # Used to get rid of duplicate reports for the same bug-type & span (macro deduplication)
                visited = set()

                # Filter UnsafeDataflow reports
                for report in filter(lambda x: x['analyzer'].startswith('UnsafeDataflow'), report_dict["reports"]):
                    if report['location'] in metadata:
                        analyzer_subcategories = report['analyzer'][16:].split('/')
                        for subcategory in analyzer_subcategories:
                            key = (report['location'], subcategory)
                            if key not in visited:
                                cnt['UnsafeDataflow'][subcategory] += 1
                                visited.add(key)
    return cnt

def count_unreported_tp(report_dir):
    assert (os.path.isdir(report_dir)), "invalid report directory"

    cnt = create_counter()

    unreported_metadata = get_unreported_metadata()

    # UnsafeDataflow
    for crate_id, metadata in unreported_metadata['UnsafeDataflow'].items():
        report_file_paths = list(glob.glob(f"{report_dir}/report-{crate_id}-*"))
        assert len(report_file_paths) >= 1, crate_id

        for report_file_path in report_file_paths:
            with open(report_file_path, "r") as report_file:
                # match this with Rudra implementation
                report_file_str = report_file.read().replace("\t", "\\t").replace("\u001B", "\\u001B")
                report_dict = tomlkit.loads(report_file_str)

                # Used to get rid of duplicate reports for the same bug-type & span (macro deduplication)
                visited = set()

                for report in filter(lambda x: x['analyzer'].startswith('UnsafeDataflow'), report_dict['reports']):
                    if report['location'] in metadata:
                        analyzer_subcategories = report['analyzer'][16:].split('/')
                        for subcategory in analyzer_subcategories:
                            key = (report['location'], subcategory)
                            if key not in visited:
                                cnt['UnsafeDataflow'][subcategory] += 1
                                visited.add(key)
    
    # SendSyncVariance
    for crate_id, metadata in unreported_metadata['SendSyncVariance'].items():
        report_file_paths = list(glob.glob(f"{report_dir}/report-{crate_id}-*"))
        assert len(report_file_paths) >= 1, crate_id

        for report_file_path in report_file_paths:
            with open(report_file_path, "r") as report_file:
                # match this with Rudra implementation
                report_file_str = report_file.read().replace("\t", "\\t").replace("\u001B", "\\u001B")
                report_dict = tomlkit.loads(report_file_str)

                # Get rid of duplicate reports for the same span (macro deduplication)
                visited_span = set()

                for report in filter(lambda x: x['analyzer'].startswith('SendSyncVariance'), report_dict['reports']):
                    if report['location'] in metadata:
                        analyzer_subcategories = report['analyzer'][18:].split('/')
                        for subcategory in analyzer_subcategories:
                            key = (report['location'], subcategory)
                            if key not in visited_span:
                                cnt['SendSyncVariance'][subcategory] += 1
                                visited_span.add(key)
    
    return cnt

if __name__ == "__main__":
    detailed_poc_metadata = get_poc_detailed_metadata()

    # Dictionary containing TruePositive counts for reported bugs
    print('TP (Reported) Count...')
    tp_cnt = count_reported_tp(
        detailed_poc_metadata,
        '/home/youngsuk/rudra-runner-home/campaign/20210425_185630/report'
    )
    print(tp_cnt)
    
    unreported_tp_cnt = count_unreported_tp('/home/youngsuk/rudra-runner-home/campaign/20210425_185630/report')
    print('TP (Unreported) Count')
    print(unreported_tp_cnt)
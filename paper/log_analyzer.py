#!/usr/bin/env python3
from common import *

import csv
import datetime
import os
import pathlib
import sys
import tomlkit
from enum import Enum, auto

# The order here should exactly match the actual analysis order of Rudra
ANALYZERS = [
    "SendSyncVariance",
    "UnsafeDataflow",
]

SUBCATEGORY = {
    "SendSyncVariance": [
        "ApiSendForSync",
        "ApiSyncforSync",
        "PhantomSendForSend",
        "NaiveSendForSend",
        "NaiveSyncForSync",
        "RelaxSend",
        "RelaxSync"
    ],
    "UnsafeDataflow": [
        "ReadFlow",
        "CopyFlow",
        "VecFromRaw",
        "Transmute",
        "WriteFlow",
        "PtrAsRef",
        "SliceUnchecked",
        "SliceFromRaw"
    ],
}

EXCLUDED_CRATES = [
    "sarekt",
    "xss-probe",
]

TARGET_START_PREFIX = "Running rudra for target "

class AnalyzerField(str, Enum):
    TIME = "time",
    NUM_REPORTS = "num_reports",
    SPAN_SET = "span_set",
    SPAN_HIGH = "span_high",
    SPAN_MED = "span_med",
    SPAN_LOW = "span_low",

class Status(Enum):
    OKAY = auto()
    EARLY_COMPILE_ERROR = auto()
    LINT_COMPILE_ERROR = auto()  # late compile error related to linting
    EMPTY_TARGET = auto()
    METADATA_ERROR = auto()
    ONLY_MAC_OS_ERROR = auto()

TIME_FORMAT = '%Y-%m-%d %H:%M:%S.%f'
TIME_LEN = len("2020-01-01 12:34:56.123456")


runner_home_dir = os.environ["RUDRA_RUNNER_HOME"]
campaign_dir = os.path.join(runner_home_dir, "campaign")

if len(sys.argv) < 2:
    print(f"Usage: {sys.argv[0]} <experiment-date>")

    print("List:")
    for entry in sorted(os.listdir(campaign_dir)):
        print(f"- {entry}")
    exit(1)

experiment_ver = sys.argv[1]
experiment_dir = os.path.join(campaign_dir, sys.argv[1])
if not os.path.exists(experiment_dir):
    print(f"`{experiment_dir}` does not exist")
    exit(1)

log_dir = os.path.join(experiment_dir, "log")
report_dir = os.path.join(experiment_dir, "report")

crate_stat = {
    "total": 0,
    "names": [],
    "stats": [],
    "status": [],
    "status_acc": {}
}

for status in list(Status):
    crate_stat["status_acc"][status] = 0

for log_file_name in os.listdir(log_dir):
    if any(map(lambda name: log_file_name.startswith(f"log-{name}"), EXCLUDED_CRATES)):
        continue

    assert_message = f"Assertion failure in {log_file_name}"
    crate_status = Status.OKAY
    cur_stat = {
        "num_target": 0,
        "e2e_time": datetime.timedelta(),  # end to end time including dependency compilation
        "rudra_time": datetime.timedelta(),  # total time taken for Rudra analysis
    }
    for analyzer in ANALYZERS:
        cur_stat[analyzer] = {
            AnalyzerField.TIME: datetime.timedelta(),
            AnalyzerField.NUM_REPORTS: 0,
            AnalyzerField.SPAN_SET: set(),
            AnalyzerField.SPAN_HIGH: set(),
            AnalyzerField.SPAN_MED: set(),
            AnalyzerField.SPAN_LOW: set(),
        }

        if analyzer in SUBCATEGORY:
            for subcategory in SUBCATEGORY[analyzer]:
                cur_stat[subcategory] = {
                    AnalyzerField.TIME: datetime.timedelta(),
                    AnalyzerField.NUM_REPORTS: 0,
                    AnalyzerField.SPAN_SET: set(),
                    AnalyzerField.SPAN_HIGH: set(),
                    AnalyzerField.SPAN_MED: set(),
                    AnalyzerField.SPAN_LOW: set(),
                }

    with open(os.path.join(log_dir, log_file_name)) as log_file:
        crate_name = log_file_name[4:]
        target_stat = None

        # I'm sorry for the quick and dirty code structure...
        prev_line = None
        for line in log_file.readlines():
            # The loop should break before reaching [stdout] line
            assert line.strip() != "[stdout]", assert_message

            if target_stat is None:
                if "Finished with non-zero exit code" in line:
                    log_file.seek(0)
                    full_log = log_file.read()
                    if "native frameworks are only available on macOS targets" in full_log:
                        crate_status = Status.ONLY_MAC_OS_ERROR
                    elif "on by default" in full_log:
                        crate_status = Status.LINT_COMPILE_ERROR
                    elif "error: trait objects without an explicit `dyn` are deprecated" in full_log:
                        crate_status = Status.LINT_COMPILE_ERROR
                    else:
                        assert False, assert_message
                    break

                if "Could not obtain Cargo metadata" in line:
                    crate_status = Status.METADATA_ERROR
                    break

                if "Running cargo rudra" in line:
                    crate_start_time = datetime.datetime.strptime(line[:TIME_LEN], TIME_FORMAT)

                idx = line.find(TARGET_START_PREFIX)
                if idx != -1:
                    # target analysis started
                    target_name = line[idx + len(TARGET_START_PREFIX):-1].replace(":", "-")
                    package_name = line[idx + len(TARGET_START_PREFIX):-1].split(":")[1]
                    report_file_name = f"report-{crate_name}-{target_name}-{package_name}"
                    analyzer_idx = -1
                    target_stat = {}

                if "cargo rudra finished" in line.strip():
                    # gracefully exit
                    crate_end_time = datetime.datetime.strptime(line[:TIME_LEN], TIME_FORMAT)
                    cur_stat["e2e_time"] = crate_end_time - crate_start_time
                    break
            else:
                if analyzer_idx == -1:
                    if "Finished with non-zero exit code" in line:
                        crate_status = Status.EARLY_COMPILE_ERROR
                        break
                    elif "cargo rudra finished" in line:
                        crate_status = Status.EMPTY_TARGET
                        break

                    assert "Rudra started" in line, assert_message
                    target_start_time = datetime.datetime.strptime(line[:TIME_LEN], TIME_FORMAT)

                    # initialize target
                    analyzer_idx = 0
                else:
                    if "Finished with non-zero exit code" in line:
                        # Analysis must not panic or timeout
                        log_file.seek(0)
                        assert False, assert_message

                    if "analysis finished" in line:
                        while ANALYZERS[analyzer_idx] not in line:
                            analyzer_idx += 1
                        analyzer = ANALYZERS[analyzer_idx]
                        assert f"{analyzer} analysis started" in prev_line, assert_message

                        target_stat[analyzer] = {}

                        # Time
                        start_time_str = prev_line[:TIME_LEN]
                        end_time_str = line[:TIME_LEN]
                        start_time = datetime.datetime.strptime(start_time_str, TIME_FORMAT)
                        end_time = datetime.datetime.strptime(end_time_str, TIME_FORMAT)
                        elapsed = end_time - start_time
                        target_stat[analyzer][AnalyzerField.TIME] = elapsed

                        # Report counting is done later
                        target_stat[analyzer][AnalyzerField.NUM_REPORTS] = 0
                        target_stat[analyzer][AnalyzerField.SPAN_SET] = set()
                        target_stat[analyzer][AnalyzerField.SPAN_HIGH] = set()
                        target_stat[analyzer][AnalyzerField.SPAN_MED] = set()
                        target_stat[analyzer][AnalyzerField.SPAN_LOW] = set()

                        # Initialize metadata for analyzer subcategories
                        if analyzer in SUBCATEGORY:
                            for subcategory in SUBCATEGORY[analyzer]:
                                # Time elapsed for subcategory is not counted.
                                target_stat[subcategory] = {
                                    AnalyzerField.TIME: datetime.timedelta(),
                                    AnalyzerField.NUM_REPORTS: 0,
                                    AnalyzerField.SPAN_SET: set(),
                                    AnalyzerField.SPAN_HIGH: set(),
                                    AnalyzerField.SPAN_MED: set(),
                                    AnalyzerField.SPAN_LOW: set(),
                                }

                    if "Rudra finished" in line:
                        target_end_time = datetime.datetime.strptime(line[:TIME_LEN], TIME_FORMAT)

                        # Report counting
                        report_file_path = os.path.join(report_dir, report_file_name)
                        if os.path.exists(report_file_path):
                            with open(report_file_path, "r") as report_file:
                                report_file_str = ansi_escape_8bit.sub("", report_file.read()).replace("\t", "\\t")
                                report_dict = tomlkit.loads(report_file_str)

                            for report in report_dict["reports"]:
                                analyzer = report['analyzer'].split(':')[0]
                                
                                assert report["level"] in ["Error", "Warning", "Info"], f"Unknown report level {report['level']}"
                                if report["level"] == "Error":
                                    report_set_field = AnalyzerField.SPAN_HIGH
                                elif report["level"] == "Warning":
                                    report_set_field = AnalyzerField.SPAN_MED
                                elif report["level"] == "Info":
                                    report_set_field = AnalyzerField.SPAN_LOW

                                target_stat[analyzer][AnalyzerField.NUM_REPORTS] += 1
                                target_stat[analyzer][AnalyzerField.SPAN_SET].add(report["location"])
                                target_stat[analyzer][report_set_field].add(report["location"])

                                if analyzer in SUBCATEGORY:
                                    for subcategory in filter(lambda x: x in report["analyzer"], SUBCATEGORY[analyzer]):
                                        target_stat[subcategory][AnalyzerField.NUM_REPORTS] += 1
                                        target_stat[subcategory][AnalyzerField.SPAN_SET].add(report["location"])
                                        target_stat[subcategory][report_set_field].add(report["location"])

                        # Accumulate target stat to crate stat
                        if crate_status == Status.OKAY:
                            cur_stat["num_target"] += 1
                            cur_stat["rudra_time"] += target_end_time - target_start_time

                            for analyzer in ANALYZERS:
                                if analyzer in target_stat:
                                    for field in AnalyzerField:
                                        if isinstance(target_stat[analyzer][field], set):
                                            cur_stat[analyzer][field] |= target_stat[analyzer][field]
                                        else:
                                            cur_stat[analyzer][field] += target_stat[analyzer][field]
                                if analyzer in SUBCATEGORY:
                                    for subcategory in SUBCATEGORY[analyzer]:
                                        for field in AnalyzerField:
                                            if isinstance(target_stat[subcategory][field], set):
                                                cur_stat[subcategory][field] |= target_stat[subcategory][field]
                                            else:
                                                cur_stat[subcategory][field] += target_stat[subcategory][field]
                        target_stat = None

            prev_line = line

    if crate_status == Status.OKAY and cur_stat["num_target"] == 0:
        crate_status = Status.EMPTY_TARGET

    crate_stat["names"].append(crate_name)
    crate_stat["stats"].append(cur_stat if crate_status == Status.OKAY else None)
    crate_stat["status"].append(crate_status)

    crate_stat["total"] += 1
    crate_stat["status_acc"][crate_status] += 1

print(crate_stat["status_acc"])

one_ms = datetime.timedelta(milliseconds=1)

total_count = {
    AnalyzerField.SPAN_HIGH: 0,
    AnalyzerField.SPAN_MED: 0,
    AnalyzerField.SPAN_LOW: 0,
}

analyzer_count = {}

for analyzer in ANALYZERS:
    analyzer_count[analyzer] = {
        AnalyzerField.SPAN_HIGH: 0,
        AnalyzerField.SPAN_MED: 0,
        AnalyzerField.SPAN_LOW: 0,
    }

# CSV export of successful crates
with open(f"stat-{sys.argv[1]}.csv", 'w', newline='') as csvfile:
    csv_writer = csv.writer(csvfile, quoting=csv.QUOTE_MINIMAL)

    header_row = ["name", "targets", "e2e-time", "rudra-time"]
    for analyzer in ANALYZERS:
        header_row.append(f"{analyzer}-time")
        header_row.append(f"{analyzer}-num-reports")
        header_row.append(f"{analyzer}-num-span")

        if analyzer in SUBCATEGORY:
            for subcategory in SUBCATEGORY[analyzer]:
                # header_row.append(f"{subcategory}-time")
                header_row.append(f"{subcategory}-num-reports")
                header_row.append(f"{subcategory}-num-span")

    header_row.append(f"total-reports")
    header_row.append(f"total-span")
    csv_writer.writerow(header_row)

    for (i, name) in enumerate(crate_stat["names"]):
        if crate_stat["status"][i] == Status.OKAY:
            stat = crate_stat["stats"][i]
            crate_row = [
                name,
                stat["num_target"],
                stat["e2e_time"] / one_ms,
                stat["rudra_time"] / one_ms,
            ]
            total_reports = 0
            total_span = 0
            for analyzer in ANALYZERS:
                crate_row.append(stat[analyzer][AnalyzerField.TIME] / one_ms)  # ms taken
                crate_row.append(stat[analyzer][AnalyzerField.NUM_REPORTS])
                crate_row.append(len(stat[analyzer][AnalyzerField.SPAN_SET]))

                total_count[AnalyzerField.SPAN_HIGH] += len(stat[analyzer][AnalyzerField.SPAN_HIGH])
                total_count[AnalyzerField.SPAN_MED] += len(stat[analyzer][AnalyzerField.SPAN_MED])
                total_count[AnalyzerField.SPAN_LOW] += len(stat[analyzer][AnalyzerField.SPAN_LOW])

                if analyzer in SUBCATEGORY:
                    analyzer_count[analyzer][AnalyzerField.SPAN_HIGH] += len(stat[analyzer][AnalyzerField.SPAN_HIGH])
                    analyzer_count[analyzer][AnalyzerField.SPAN_MED] += len(stat[analyzer][AnalyzerField.SPAN_MED])
                    analyzer_count[analyzer][AnalyzerField.SPAN_LOW] += len(stat[analyzer][AnalyzerField.SPAN_LOW])

                    for subcategory in SUBCATEGORY[analyzer]:
                        # crate_row.append(stat[subcategory][AnalyzerField.TIME] / one_ms)  # ms taken
                        crate_row.append(stat[subcategory][AnalyzerField.NUM_REPORTS])
                        crate_row.append(len(stat[subcategory][AnalyzerField.SPAN_SET]))

                total_reports += stat[analyzer][AnalyzerField.NUM_REPORTS]
                total_span += len(stat[analyzer][AnalyzerField.SPAN_SET])
            crate_row.append(total_reports)
            crate_row.append(total_span)
            csv_writer.writerow(crate_row)

with open(f"status-{sys.argv[1]}.csv", 'w', newline='') as csvfile:
    csv_writer = csv.writer(csvfile, quoting=csv.QUOTE_MINIMAL)

    csv_writer.writerow(("name", "status"))
    for (i, name) in enumerate(crate_stat["names"]):
        csv_writer.writerow((name, crate_stat["status"][i]))


# Standard library analysis
std_location_set = set()

for report_file_path in (PROJECT_DIRECTORY / "stdlib-analysis").glob("report-*"):
    with open(report_file_path, "r") as report_file:
        report_file_str = ansi_escape_8bit.sub("", report_file.read())
        report_dict = tomlkit.loads(report_file_str)

    for report in report_dict["reports"]:
        analyzer = report['analyzer'].split(':')[0]

        assert report["level"] in ["Error", "Warning", "Info"], f"Unknown report level {report['level']}"
        if report["level"] == "Error":
            report_set_field = AnalyzerField.SPAN_HIGH
        elif report["level"] == "Warning":
            report_set_field = AnalyzerField.SPAN_MED
        elif report["level"] == "Info":
            report_set_field = AnalyzerField.SPAN_LOW

        if report["location"] not in std_location_set:
            std_location_set.add(report["location"])
            analyzer_count[analyzer][report_set_field] += 1


for analyzer in ANALYZERS:
    print(f"Reports for analyzer {analyzer}")

    high_count = analyzer_count[analyzer][AnalyzerField.SPAN_HIGH]
    med_count = analyzer_count[analyzer][AnalyzerField.SPAN_MED]
    low_count = analyzer_count[analyzer][AnalyzerField.SPAN_LOW]

    print(f"  On high: {high_count}")
    print(f"  On med: {high_count + med_count}")
    print(f"  On low: {high_count + med_count + low_count}")

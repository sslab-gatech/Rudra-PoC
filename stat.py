#!/usr/bin/env python3
import csv
import datetime
import os
import sys
import tomlkit
from enum import Enum, auto

# The order here should exactly match the actual analysis order of Rudra
ANALYZERS = [
    "CallGraph",  # will be removed in the future run
    "UnsafeDestructor",
]

TARGET_START_PREFIX = "Running rudra for target "

class AnalyzerField(str, Enum):
    TIME = "time",
    NUM_REPORTS = "num_reports",

class Status(Enum):
    OKAY = auto()
    EARLY_COMPILE_ERROR = auto()
    TYPE_COMPILE_ERROR = auto()  # late compile error related to deep type
    LINT_COMPILE_ERROR = auto()  # late compile error related to linting
    EMPTY_TARGET = auto()
    METADATA_ERROR = auto()
    RLIB_ERROR = auto()  # this was mistake on my side, it should become OKAY after fix
    ICE_TYPECK_ERROR = auto()  # this was mistake on my side, it should become OKAY after fix
    ICE_IN_CALL_GRAPH_ERROR = auto()  # this was mistake on my side, it should become OKAY after fix
    ONLY_MAC_OS_ERROR = auto()

TIME_FORMAT = '%Y-%m-%d %H:%M:%S.%f'
TIME_LEN = len("2020-01-01 12:34:56.123456")


report_dir = os.environ["RUDRA_REPORT_DIR"]

if len(sys.argv) < 2:
    print(f"Usage: {sys.argv[0]} <experiment-date>")

    print("List:")
    for entry in os.listdir(report_dir):
        print(f"- {entry}")
    exit(1)

experiment_ver = sys.argv[1]
experiment_dir = os.path.join(report_dir, sys.argv[1])
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
    assert_message = f"Assertion failure in {log_file_name}"
    crate_status = Status.OKAY
    cur_stat = {
        "num_target": 0
    }
    for analyzer in ANALYZERS:
        cur_stat[analyzer] = {
            AnalyzerField.TIME: datetime.timedelta(),
            AnalyzerField.NUM_REPORTS: 0,
        }

    with open(os.path.join(log_dir, log_file_name)) as log_file:
        crate_name = log_file_name[4:]
        target_stat = None

        # I'm sorry for the quick and dirty code structure...
        prev_line = None
        for line in log_file.readlines():
            assert line.strip() != "[stdout]", assert_message

            if target_stat is None:
                if "Finished with non-zero exit code" in line:
                    log_file.seek(0)
                    full_log = log_file.read()
                    if "extern location for" in full_log:
                        crate_status = Status.RLIB_ERROR
                    elif "native frameworks are only available on macOS targets" in full_log:
                        crate_status = Status.ONLY_MAC_OS_ERROR
                    elif ("reached the type-length limit while instantiating" in full_log
                        or "overflow representing the type" in full_log):
                        crate_status = Status.TYPE_COMPILE_ERROR
                    elif "on by default" in full_log:
                        crate_status = Status.LINT_COMPILE_ERROR
                    else:
                        assert False, assert_message
                    break

                if "Could not obtain Cargo metadata" in line:
                    crate_status = Status.METADATA_ERROR
                    break

                idx = line.find(TARGET_START_PREFIX)
                if idx != -1:
                    # target analysis started
                    target_name = line[idx + len(TARGET_START_PREFIX):-1].replace(":", "-")
                    report_file_name = f"report-{crate_name}-{target_name}"
                    analyzer_idx = -1
                    target_stat = {}

                if "cargo rudra finished" in line.strip():
                    # gracefully exit
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

                    # initialize target
                    analyzer_idx = 0
                else:
                    if "Finished with non-zero exit code" in line:
                        log_file.seek(0)
                        full_log = log_file.read()
                        if "CallGraph analysis started" in prev_line:
                            crate_status = Status.ICE_IN_CALL_GRAPH_ERROR
                        elif "can't type-check body of" in full_log:
                            crate_status = Status.ICE_TYPECK_ERROR
                        else:
                            assert False, assert_message
                        break

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

                    if "Rudra finished" in line:
                        # Report counting
                        report_file_path = os.path.join(report_dir, report_file_name)
                        if os.path.exists(report_file_path):
                            with open(report_file_path, "r") as report_file:
                                report_dict = tomlkit.loads(report_file.read())
                            for report in report_dict["reports"]:
                                target_stat[report["analyzer"]][AnalyzerField.NUM_REPORTS] += 1

                        # Accumulate target stat to crate stat
                        if crate_status == Status.OKAY:
                            cur_stat["num_target"] += 1
                            for analyzer in ANALYZERS:
                                if analyzer in target_stat:
                                    for field in AnalyzerField:
                                        cur_stat[analyzer][field] += target_stat[analyzer][field]
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

# TODO: measure end-to-end execution time

# CSV export of successful crates
with open(f"stat-{sys.argv[1]}.csv", 'w', newline='') as csvfile:
    csv_writer = csv.writer(csvfile, quoting=csv.QUOTE_MINIMAL)

    header_row = ["name", "targets"]
    for analyzer in ANALYZERS:
        header_row.append(f"{analyzer}-time")
        header_row.append(f"{analyzer}-num-reports")
    csv_writer.writerow(header_row)

    for (i, name) in enumerate(crate_stat["names"]):
        if crate_stat["status"][i] == Status.OKAY:
            stat = crate_stat["stats"][i]
            crate_row = [name, stat["num_target"]]
            for analyzer in ANALYZERS:
                crate_row.append(stat[analyzer][AnalyzerField.TIME] / datetime.timedelta(milliseconds=1))  # ms taken
                crate_row.append(stat[analyzer][AnalyzerField.NUM_REPORTS])
            csv_writer.writerow(crate_row)

with open(f"status-{sys.argv[1]}.csv", 'w', newline='') as csvfile:
    csv_writer = csv.writer(csvfile, quoting=csv.QUOTE_MINIMAL)

    csv_writer.writerow(("name", "status"))
    for (i, name) in enumerate(crate_stat["names"]):
        csv_writer.writerow((name, crate_stat["status"][i]))

#!/usr/bin/env python3
import common
import count_by_rank
from count_by_rank import Rank

import csv
import datetime
import glob
import os
import pandas as pd
import sys
import tomlkit
from enum import IntEnum, Enum, auto

# The order here should exactly match the actual analysis order of Rudra
ANALYZERS = [
    "UnsafeDestructor",
    "SendSyncVariance",
    "UnsafeDataflow",
]


RANK_CATEGORIES = [
    "UnsafeDataflow/0",
    "UnsafeDataflow/1",
    "UnsafeDataflow/2",
    "SendSyncVariance/0",
    "SendSyncVariance/1",
    "SendSyncVariance/2",
]

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
        "SliceFromRaw",
        "VecSetLen"
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

class Status(Enum):
    OKAY = auto()
    EARLY_COMPILE_ERROR = auto()
    TYPE_COMPILE_ERROR = auto()  # late compile error related to deep type
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
        }

        for rank in RANK_CATEGORIES:
            cur_stat[rank] = {
                AnalyzerField.TIME: datetime.timedelta(),
                AnalyzerField.NUM_REPORTS: 0,
                AnalyzerField.SPAN_SET: set(),
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
                    elif ("reached the type-length limit while instantiating" in full_log
                        or "overflow representing the type" in full_log):
                        crate_status = Status.TYPE_COMPILE_ERROR
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
                    report_file_name = f"report-{crate_name}-{target_name}"
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

                        # Initialize metadata for analyzer subcategories
                        if analyzer in SUBCATEGORY:
                            for rank in Rank:
                                # Time elapsed for subcategory is not counted.
                                target_stat[f'{analyzer}/{rank}'] = {
                                    AnalyzerField.TIME: datetime.timedelta(),
                                    AnalyzerField.NUM_REPORTS: 0,
                                    AnalyzerField.SPAN_SET: set()
                                }

                    if "Rudra finished" in line:
                        target_end_time = datetime.datetime.strptime(line[:TIME_LEN], TIME_FORMAT)

                        # Report counting
                        report_file_paths = glob.glob(f"{os.path.join(report_dir, report_file_name)}-*")
                        for report_file_path in report_file_paths:
                            with open(report_file_path, "r") as report_file:
                                # match this with Rudra implementation
                                report_file_str = report_file.read().replace("\t", "\\t").replace("\u001B", "\\u001B")
                                report_dict = tomlkit.loads(report_file_str)
                            for report in report_dict["reports"]:
                                tmp = report['analyzer'].split(':/')
                                analyzer = tmp[0]
                                target_stat[analyzer][AnalyzerField.NUM_REPORTS] += 1
                                target_stat[analyzer][AnalyzerField.SPAN_SET].add(report["location"])

                                if analyzer in SUBCATEGORY:
                                    for subcategory in tmp[1].split('/'):
                                        span, cur_rank = report['location'], HASHER[subcategory]
                                        for rank in Rank:
                                            if cur_rank >= rank:
                                                target_stat[f'{analyzer}/{rank}'][AnalyzerField.SPAN_SET].add(span)

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
                                    for rank in Rank:
                                        key = f'{analyzer}/{rank}'
                                        for field in AnalyzerField:
                                            if isinstance(target_stat[key][field], set):
                                                cur_stat[key][field] |= target_stat[key][field]
                                            else:
                                                cur_stat[key][field] += target_stat[key][field]
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

# CSV export of successful crates
with open(f"stat-{sys.argv[1]}-rank.csv", 'w', newline='') as csvfile:
    csv_writer = csv.writer(csvfile, quoting=csv.QUOTE_MINIMAL)

    header_row = ["name", "targets", "e2e-time", "rudra-time"]
    for analyzer in ANALYZERS:
        header_row.append(f"{analyzer}-time")
        header_row.append(f"{analyzer}-num-reports")
        header_row.append(f"{analyzer}-num-span")

        if analyzer in SUBCATEGORY:
            for rank in Rank:
                key = f'{analyzer}/{rank}'
                header_row.append(f"{key}-num-span")

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

                if analyzer in SUBCATEGORY:
                    for rank in Rank:
                        key = f'{analyzer}/{rank}'
                        crate_row.append(len(stat[key][AnalyzerField.SPAN_SET]))

                total_reports += stat[analyzer][AnalyzerField.NUM_REPORTS]
                total_span += len(stat[analyzer][AnalyzerField.SPAN_SET])
            crate_row.append(total_reports)
            crate_row.append(total_span)
            csv_writer.writerow(crate_row)

with open(f"status-{sys.argv[1]}-rank.csv", 'w', newline='') as csvfile:
    csv_writer = csv.writer(csvfile, quoting=csv.QUOTE_MINIMAL)

    csv_writer.writerow(("name", "status"))
    for (i, name) in enumerate(crate_stat["names"]):
        csv_writer.writerow((name, crate_stat["status"][i]))

tp_cnt = count_by_rank.count_reported_tp(
    common.get_poc_detailed_metadata(),
    f'{campaign_dir}/{experiment_ver}/report',
)
unreported_tp_cnt = count_by_rank.count_unreported_tp(
    f'{campaign_dir}/{experiment_ver}/report',
)

stat = pd.read_csv(f'stat-{experiment_ver}-rank.csv')

# Don't forget to take into account 3 additional bugs that are not represented as pocs in Rudra-PoC.
stat['total-span'][0] += 3
# https://github.com/rust-lang/rust/issues/80894 => VecSetLen (Rank.HIGH)
# https://github.com/rust-lang/rust/issues/80335 => SliceUnchecked (Rank.LOW)
# => Add 2 to Rank.LOW, Add 1 to Rank.MID, Add 1 to Rank.HIGH
tp_cnt['UnsafeDataflow'][Rank.LOW] += 2
tp_cnt['UnsafeDataflow'][Rank.MID] += 1
tp_cnt['UnsafeDataflow'][Rank.HIGH] += 1
stat['UnsafeDataflow-num-span'][0] += 2
stat['UnsafeDataflow/2-num-span'][0] += 1
stat['UnsafeDataflow/1-num-span'][0] += 1
stat['UnsafeDataflow/0-num-span'][0] += 2
# https://github.com/rust-lang/rust/issues/81425 => ApiSyncforSync/NaiveSyncForSync/RelaxSync (Rank.MID/LOW)
# => Add 1 to Rank.MID, Add 1 to Rank.LOW
tp_cnt['SendSyncVariance'][Rank.LOW] += 1
tp_cnt['SendSyncVariance'][Rank.MID] += 1
stat['SendSyncVariance-num-span'][0] += 1
stat['SendSyncVariance/1-num-span'][0] += 1
stat['SendSyncVariance/0-num-span'][0] += 1

print("""
\\newcommand{\\cratesioCrate}{\\num{%d}\\xspace}
\\newcommand{\\cratesioCrateShort}{%dk\\xspace}

\\newcommand{\\totalTimeHours}{%d hours\\xspace}

\\newcommand{\\compileOkay}{\\num{%d}\\xspace}
\\newcommand{\\compileError}{\\num{%d}\\xspace}
\\newcommand{\\compileEmpty}{\\num{%d}\\xspace}
\\newcommand{\\compileMetadataError}{\\num{%d}\\xspace}

\\newcommand{\\elapsedTotalAverage}{\\num{%d}~sec\\xspace}
\\newcommand{\\elapsedRudraAverage}{\\num{%d}~ms\\xspace}
\\newcommand{\\elapsedSvAverage}{\\num{%d}~ms\\xspace}
\\newcommand{\\elapsedUdAverage}{\\num{%d}~ms\\xspace}

\\newcommand{\\reportsAll}{\\num{%d}\\xspace}

\\newcommand{\\udReportsAll}{\\num{%d}\\xspace}
\\newcommand{\\udReportsHigh}{\\num{%d}\\xspace}
\\newcommand{\\udReportsMed}{\\num{%d}\\xspace}
\\newcommand{\\udReportsLow}{\\num{%d}\\xspace}

\\newcommand{\\udBugsAll}{\\num{%d}\\xspace}
\\newcommand{\\udBugsHigh}{\\num{%d}\\xspace}
\\newcommand{\\udBugsMed}{\\num{%d}\\xspace}
\\newcommand{\\udBugsLow}{\\num{%d}\\xspace}

\\newcommand{\\udBugsAllPercentage}{%d\\%%\\xspace}
\\newcommand{\\udBugsHighPercentage}{%d\\%%\\xspace}
\\newcommand{\\udBugsMedPercentage}{%d\\%%\\xspace}
\\newcommand{\\udBugsLowPercentage}{%d\\%%\\xspace}

\\newcommand{\\udInternalAll}{\\num{%d}\\xspace}
\\newcommand{\\udInternalHigh}{\\num{%d}\\xspace}
\\newcommand{\\udInternalMed}{\\num{%d}\\xspace}
\\newcommand{\\udInternalLow}{\\num{%d}\\xspace}

\\newcommand{\\udInternalAllPercentage}{%d\\%%\\xspace}
\\newcommand{\\udInternalHighPercentage}{%d\\%%\\xspace}
\\newcommand{\\udInternalMedPercentage}{%d\\%%\\xspace}
\\newcommand{\\udInternalLowPercentage}{%d\\%%\\xspace}

\\newcommand{\\udBugSumAll}{\\num{%d}\\xspace}
\\newcommand{\\udBugSumHigh}{\\num{%d}\\xspace}
\\newcommand{\\udBugSumMed}{\\num{%d}\\xspace}
\\newcommand{\\udBugSumLow}{\\num{%d}\\xspace}

\\newcommand{\\udBugSumAllPercentage}{%d\\%%\\xspace}
\\newcommand{\\udBugSumHighPercentage}{%d\\%%\\xspace}
\\newcommand{\\udBugSumMedPercentage}{%d\\%%\\xspace}
\\newcommand{\\udBugSumLowPercentage}{%d\\%%\\xspace}

\\newcommand{\\svReportsAll}{\\num{%d}\\xspace}
\\newcommand{\\svReportsHigh}{\\num{%d}\\xspace}
\\newcommand{\\svReportsMed}{\\num{%d}\\xspace}
\\newcommand{\\svReportsLow}{\\num{%d}\\xspace}

\\newcommand{\\svBugsAll}{\\num{%d}\\xspace}
\\newcommand{\\svBugsHigh}{\\num{%d}\\xspace}
\\newcommand{\\svBugsMed}{\\num{%d}\\xspace}
\\newcommand{\\svBugsLow}{\\num{%d}\\xspace}

\\newcommand{\\svBugsAllPercentage}{%d\\%%\\xspace}
\\newcommand{\\svBugsHighPercentage}{%d\\%%\\xspace}
\\newcommand{\\svBugsMedPercentage}{%d\\%%\\xspace}
\\newcommand{\\svBugsLowPercentage}{%d\\%%\\xspace}

\\newcommand{\\svInternalAll}{\\num{%d}\\xspace}
\\newcommand{\\svInternalHigh}{\\num{%d}\\xspace}
\\newcommand{\\svInternalMed}{\\num{%d}\\xspace}
\\newcommand{\\svInternalLow}{\\num{%d}\\xspace}

\\newcommand{\\svInternalAllPercentage}{%d\\%%\\xspace}
\\newcommand{\\svInternalHighPercentage}{%d\\%%\\xspace}
\\newcommand{\\svInternalMedPercentage}{%d\\%%\\xspace}
\\newcommand{\\svInternalLowPercentage}{%d\\%%\\xspace}

\\newcommand{\\svBugSumAll}{\\num{%d}\\xspace}
\\newcommand{\\svBugSumHigh}{\\num{%d}\\xspace}
\\newcommand{\\svBugSumMed}{\\num{%d}\\xspace}
\\newcommand{\\svBugSumLow}{\\num{%d}\\xspace}

\\newcommand{\\svBugSumAllPercentage}{%d\\%%\\xspace}
\\newcommand{\\svBugSumHighPercentage}{%d\\%%\\xspace}
\\newcommand{\\svBugSumMedPercentage}{%d\\%%\\xspace}
\\newcommand{\\svBugSumLowPercentage}{%d\\%%\\xspace}
""" % (
    len(stat['name']), # Total Crates
    len(stat['name']) // 1000, # 1000,

    round(stat['e2e-time'].sum() / (3600000), 1), # milliseconds to hour conversion
    
    crate_stat['status_acc'][Status.OKAY], # Compile Okay
    crate_stat['status_acc'][Status.EARLY_COMPILE_ERROR] + crate_stat['status_acc'][Status.TYPE_COMPILE_ERROR] + crate_stat['status_acc'][Status.LINT_COMPILE_ERROR] + crate_stat['status_acc'][Status.ONLY_MAC_OS_ERROR], # Compile Error
    crate_stat['status_acc'][Status.EMPTY_TARGET], # Compile Empty
    crate_stat['status_acc'][Status.METADATA_ERROR], # compileMetadataError

    stat['e2e-time'].mean() / 1000, # elapsedTotalAverage (s)
    stat['rudra-time'].mean(), # elapsedRudraAverage (ms)
    stat['SendSyncVariance-time'].sum() / crate_stat['status_acc'][Status.OKAY], # elapsedSvAverage
    stat['UnsafeDataflow-time'].sum() / crate_stat['status_acc'][Status.OKAY], # elapsedUdAverage

    # Total Count (of reports)
    stat['total-span'].sum(),

    # UD Count
    stat['UnsafeDataflow-num-span'].sum(), # udReportsAll
    stat['UnsafeDataflow/2-num-span'].sum(),
    stat['UnsafeDataflow/1-num-span'].sum(),
    stat['UnsafeDataflow/0-num-span'].sum(),

    tp_cnt['UnsafeDataflow'][Rank.LOW], # UDBugsAll
    tp_cnt['UnsafeDataflow'][Rank.HIGH], # UDBugsHigh
    tp_cnt['UnsafeDataflow'][Rank.MID], # UDBugsMed
    tp_cnt['UnsafeDataflow'][Rank.LOW], # UDBugsLow
    
    round(tp_cnt['UnsafeDataflow'][Rank.LOW]/stat['UnsafeDataflow-num-span'].sum() * 100, 1), # udBugsAllPercentage
    round(tp_cnt['UnsafeDataflow'][Rank.HIGH]/stat['UnsafeDataflow/2-num-span'].sum() * 100, 1), # udBugsHighPercentage
    round(tp_cnt['UnsafeDataflow'][Rank.MID]/stat['UnsafeDataflow/1-num-span'].sum() * 100, 1), # udBugsMedPercentage
    round(tp_cnt['UnsafeDataflow'][Rank.LOW]/stat['UnsafeDataflow/0-num-span'].sum() * 100, 1), # udBugsLowPercentage

    unreported_tp_cnt['UnsafeDataflow'][Rank.LOW], # udInternalAll
    unreported_tp_cnt['UnsafeDataflow'][Rank.HIGH], # udInternalHigh
    unreported_tp_cnt['UnsafeDataflow'][Rank.MID], # udInternalMed
    unreported_tp_cnt['UnsafeDataflow'][Rank.LOW], # udInternalLow

    # udInternalAllPercentage
    round(unreported_tp_cnt['UnsafeDataflow'][Rank.LOW]/stat['UnsafeDataflow-num-span'].sum() * 100, 1),
    round(unreported_tp_cnt['UnsafeDataflow'][Rank.HIGH]/stat['UnsafeDataflow/2-num-span'].sum() * 100, 1),
    round(unreported_tp_cnt['UnsafeDataflow'][Rank.MID]/stat['UnsafeDataflow/1-num-span'].sum() * 100, 1),
    round(unreported_tp_cnt['UnsafeDataflow'][Rank.LOW]/stat['UnsafeDataflow/0-num-span'].sum() * 100, 1),

    # udBugSumAll
    tp_cnt['UnsafeDataflow'][Rank.LOW] + unreported_tp_cnt['UnsafeDataflow'][Rank.LOW],
    tp_cnt['UnsafeDataflow'][Rank.HIGH] + unreported_tp_cnt['UnsafeDataflow'][Rank.HIGH],
    tp_cnt['UnsafeDataflow'][Rank.MID] + unreported_tp_cnt['UnsafeDataflow'][Rank.MID],
    tp_cnt['UnsafeDataflow'][Rank.LOW] + unreported_tp_cnt['UnsafeDataflow'][Rank.LOW],

    # udBugSumAllPercentage
    round((tp_cnt['UnsafeDataflow'][Rank.LOW] + unreported_tp_cnt['UnsafeDataflow'][Rank.LOW]) / stat['UnsafeDataflow-num-span'].sum() * 100, 1),
    round((tp_cnt['UnsafeDataflow'][Rank.HIGH] + unreported_tp_cnt['UnsafeDataflow'][Rank.HIGH]) / stat['UnsafeDataflow/2-num-span'].sum() * 100, 1),
    round((tp_cnt['UnsafeDataflow'][Rank.MID] + unreported_tp_cnt['UnsafeDataflow'][Rank.MID]) / stat['UnsafeDataflow/1-num-span'].sum() * 100, 1),
    round((tp_cnt['UnsafeDataflow'][Rank.LOW] + unreported_tp_cnt['UnsafeDataflow'][Rank.HIGH]) / stat['UnsafeDataflow/0-num-span'].sum() * 100, 1),

    # SV Count
    stat['SendSyncVariance-num-span'].sum(),
    stat['SendSyncVariance/2-num-span'].sum(),
    stat['SendSyncVariance/1-num-span'].sum(),
    stat['SendSyncVariance/0-num-span'].sum(),
    
    tp_cnt['SendSyncVariance'][Rank.LOW], # SVBugsAll
    tp_cnt['SendSyncVariance'][Rank.HIGH], # SVBugsHigh
    tp_cnt['SendSyncVariance'][Rank.MID], # SVBugsMed
    tp_cnt['SendSyncVariance'][Rank.LOW], # SVBugsLow

    round(tp_cnt['SendSyncVariance'][Rank.LOW]/stat['SendSyncVariance-num-span'].sum() * 100, 1), # svBugsAllPercentage
    round(tp_cnt['SendSyncVariance'][Rank.HIGH]/stat['SendSyncVariance/2-num-span'].sum() * 100, 1), # svBugsHighPercentage
    round(tp_cnt['SendSyncVariance'][Rank.MID]/stat['SendSyncVariance/1-num-span'].sum() * 100, 1), # svBugsMedPercentage
    round(tp_cnt['SendSyncVariance'][Rank.LOW]/stat['SendSyncVariance/0-num-span'].sum() * 100, 1), # svBugsLowPercentage

    # svInternalAll
    unreported_tp_cnt['SendSyncVariance'][Rank.LOW], # svInternalAll
    unreported_tp_cnt['SendSyncVariance'][Rank.HIGH], # svInternalHigh
    unreported_tp_cnt['SendSyncVariance'][Rank.MID], # svInternalMed
    unreported_tp_cnt['SendSyncVariance'][Rank.LOW], # svInternalLow

    # svInternalAllPercentage
    round(unreported_tp_cnt['SendSyncVariance'][Rank.LOW]/stat['SendSyncVariance-num-span'].sum() * 100, 1),
    round(unreported_tp_cnt['SendSyncVariance'][Rank.HIGH]/stat['SendSyncVariance/2-num-span'].sum() * 100, 1),
    round(unreported_tp_cnt['SendSyncVariance'][Rank.MID]/stat['SendSyncVariance/1-num-span'].sum() * 100, 1),
    round(unreported_tp_cnt['SendSyncVariance'][Rank.LOW]/stat['SendSyncVariance/0-num-span'].sum() * 100, 1),

    # svBugSumAll
    tp_cnt['SendSyncVariance'][Rank.LOW] + unreported_tp_cnt['SendSyncVariance'][Rank.LOW],
    tp_cnt['SendSyncVariance'][Rank.HIGH] + unreported_tp_cnt['SendSyncVariance'][Rank.HIGH],
    tp_cnt['SendSyncVariance'][Rank.MID] + unreported_tp_cnt['SendSyncVariance'][Rank.MID],
    tp_cnt['SendSyncVariance'][Rank.LOW] + unreported_tp_cnt['SendSyncVariance'][Rank.LOW],

    # svBugSumAllPercentage
    round((tp_cnt['SendSyncVariance'][Rank.LOW] + unreported_tp_cnt['SendSyncVariance'][Rank.LOW]) / stat['SendSyncVariance-num-span'].sum() * 100, 1),
    round((tp_cnt['SendSyncVariance'][Rank.HIGH] + unreported_tp_cnt['SendSyncVariance'][Rank.HIGH]) / stat['SendSyncVariance/2-num-span'].sum() * 100, 1),
    round((tp_cnt['SendSyncVariance'][Rank.MID] + unreported_tp_cnt['SendSyncVariance'][Rank.MID]) / stat['SendSyncVariance/1-num-span'].sum() * 100, 1),
    round((tp_cnt['SendSyncVariance'][Rank.LOW] + unreported_tp_cnt['SendSyncVariance'][Rank.LOW]) / stat['SendSyncVariance/0-num-span'].sum() * 100, 1),
))

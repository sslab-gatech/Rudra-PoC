import os
import sys
from enum import Enum, auto

# The order here should exactly match the actual analysis order of Rudra
ANALYZERS = [
    "CallGraph",  # will be removed in the future run
    "UnsafeDestructor",
]

ANALYZER_FIELDS = [
    "time",
    "num_reports",
]

TARGET_START_PREFIX = "Running rudra for target "

class Status(Enum):
    OKAY = auto()
    COMPILE_ERROR = auto()
    TYPE_COMPILE_ERROR = auto()
    LINT_COMPILE_ERROR = auto()
    EMPTY_COMPILE_ERROR = auto()
    METADATA_ERROR = auto()
    RLIB_ERROR = auto()  # this was mistake on my side, it should become OKAY after fix
    ICE_TYPECK_ERROR = auto()  # this was mistake on my side, it should become OKAY after fix
    ICE_IN_CALL_GRAPH_ERROR = auto()  # this was mistake on my side, it should become OKAY after fix
    ONLY_MAC_OS_ERROR = auto()


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

# TODO: time parsing
# TODO: report count
for log_file_name in os.listdir(log_dir):
    assert_message = f"Assertion failure in {log_file_name}"
    crate_status = Status.OKAY
    cur_stat = {}
    for analyzer in ANALYZERS:
        cur_stat[analyzer] = {}
        for field in ANALYZER_FIELDS:
            cur_stat[analyzer][field] = 0

    with open(os.path.join(log_dir, log_file_name)) as log_file:
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
                    target_name = line[idx + len(TARGET_START_PREFIX):]
                    analyzer_idx = -1
                    target_stat = {}
                
                if "cargo rudra finished" in line.strip():
                    # gracefully exit
                    break
            else:
                if analyzer_idx == -1:
                    if "Finished with non-zero exit code" in line:
                        crate_status = Status.COMPILE_ERROR
                        break
                    elif "cargo rudra finished" in line:
                        crate_status = Status.EMPTY_COMPILE_ERROR
                        break

                    assert "Rudra started" in line, assert_message

                    # initialize target
                    analyzer_idx = 0
                    for analyzer in ANALYZERS:
                        target_stat[analyzer] = {}
                        for field in ANALYZER_FIELDS:
                            target_stat[analyzer][field] = 0
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

                    if "Rudra finished" in line:
                        # Accumulate target stat to crate stat
                        for analyzer in ANALYZERS:
                            for field in ANALYZER_FIELDS:
                                cur_stat[analyzer][field] += target_stat[analyzer][field]
                        target_stat = None

            prev_line = line

    crate_stat["names"].append(log_file_name)
    crate_stat["stats"].append(cur_stat if crate_status == Status.OKAY else None)
    crate_stat["status"].append(crate_status)

    crate_stat["total"] += 1
    crate_stat["status_acc"][crate_status] += 1

print(crate_stat["status_acc"])

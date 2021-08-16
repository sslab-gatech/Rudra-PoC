#!/usr/bin/env python3
import csv

MIN_YEAR = 2016
MAX_YEAR = 2021

ours_total = 0

with open("rustsec_list_annotated.csv", "r", newline="") as csvfile:
    csv_reader = csv.reader(csvfile)

    count_all = {
        'logic': 0,
        'unmaintained': 0,
        'unsafe': 0,
        'notice': 0,
        'ignore': 0,
        'unsafe_ours': 0,
    }

    count_dict = {}
    for year in range(MIN_YEAR, MAX_YEAR + 1):
        count_dict[str(year)] = {
            'logic': 0,
            'unmaintained': 0,
            'unsafe': 0,
            'notice': 0,
            'ignore': 0,
            'unsafe_ours': 0,
        }

    # skip header
    next(csv_reader, None)
    for row in csv_reader:
        (id, year, type, ours) = row
        meta = count_dict[year]
        meta[type] += 1
        count_all[type] += 1
        if ours == "TRUE":
            ours_total += 1
            if type == "logic":
                print(f"Warning: {id} has logic category!")
            elif type == "unsafe":
                meta["unsafe_ours"] += 1
                count_all["unsafe_ours"] += 1
            elif type == "unmaintained":
                # Note: Some of our bugs have real memory safety issues but categorized as "unmaintained"
                # We are not counting them as memory safety bugs,
                # which is more conservative way to count # of bugs found by Rudra.
                pass
            else:
                raise Exception("Unknown `type` value with ours!")
        elif ours != "FALSE":
            raise Exception("Unknown `ours` value!")

year_list = list(map(str, range(MIN_YEAR, MAX_YEAR+1)))

unsafe_ours = count_all['unsafe_ours']

print(f"Rudra-found RustSec memory-safety bugs: {unsafe_ours}")
print(f"RustSec total: {count_all['logic'] + count_all['unsafe']}")
print(f"RustSec memory-safety: {count_all['unsafe']}")
print("")
print(f"RustSec total %: {100. * unsafe_ours / (count_all['logic'] + count_all['unsafe']):.1f}%")
print(f"RustSec memory-safety %: {100. * unsafe_ours / count_all['unsafe']:.1f}%")

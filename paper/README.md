# Rudra Paper Scripts

This folder contains convenience scripts for the Rudra paper. Crates in
`metadata.csv` appear in the table as per the original order.

## Installation

1. `python3 -m venv venv`

2. `source venv/bin/activate`

3. `python3 -m pip install -r requirements.txt`

## Description

* `common.py` - Library for other scripts, do not run as a standalone script.
* `count_bugs.py` - Count number of bugs for each analyzer, total CVE, etc.
* `generate_table.py` - Generate a large table of the most prominent bugs
* `log_analyzer.py` - How much time was taken in each step, how many crates compiled, etc.
    * TODO: Needs rework based on the updated directory structure
* `recreate_bugs.py` - Verify the current version of Rudra still detects the previously found bugs.
* `rustsec_list.py` - Generate `rustsec_list.csv`. Add manual triage information to the generated file,
    copy that file to the paper repository, and run `rustsec_list_counter.py` there.
* `verify.py` - Sanity checker for PoCs.

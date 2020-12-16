#!/usr/bin/env python3
import subprocess
import glob
import time
import concurrent.futures
import multiprocessing
import os
import sys
import json


def run_poc(poc_id):
    p = subprocess.Popen([
        'cli/target/release/rudra-poc', 'run', '%04d' % (poc_id)
    ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    t = 0
    while True:
        try:
            p.wait(1)
            break
        except subprocess.TimeoutExpired:
            pass
        t += 1
        name = "target/debug/rudra-poc-%04d" % poc_id
        ps_output = subprocess.check_output("ps a | grep %s | grep -v rustc | grep -v grep ; exit 0" % name, shell=True).decode('ascii')
        poc_running = name in ps_output
        # print(poc_id, ps_output)
        if not poc_running:
            t = 0
        if t > 180:
            p.terminate()
            try:
                p.wait(5)
            except subprocess.TimeoutExpired:
                p.kill()
            p.wait()
            return 'FAIL (hang)'
    o, e = p.communicate()
    outputs = e.decode('ascii') + o.decode('ascii')
    if "error: failed to select a version for the requirement" in outputs or \
        'error: no matching package named' in outputs:
        return 'FAIL (crate yanked?)'
    elif 'error: build failed\n' in outputs or \
        'error: linking with `cc` failed' in outputs or \
        'Error: command ["cargo", "build"] exited with code' in outputs:
        print(outputs)
        return 'FAIL (build failed)'
    elif 'Terminated with signal' in outputs or \
        'Return code 101\n' in outputs or \
        "thread 'main' panicked at" in outputs or \
        "thread '<unnamed>' panicked at" in outputs:
        return 'OK'
    elif 'Return code 0' in outputs:
        return 'FAIL (clean exit)'
    else:
        return 'FAIL ```%s```' % outputs
    


if __name__ == '__main__':
    CI = 'CI' in os.environ
    push_files = set()
    if CI:
        event = json.loads(open(os.environ['GITHUB_EVENT_PATH']).read())
        for f in subprocess.check_output(["git", "diff", "--name-only", event["before"], event["after"]]).decode('ascii').splitlines():
            push_files.add(f)
    POCS = {}
    for f in sorted(glob.glob('poc/*-*.rs')):
        if CI and f not in push_files:
            continue
        f = f.split('/')[1]
        poc_id = int(f.split('-')[0])
        POCS[poc_id] = f

    all_good = True
    with concurrent.futures.ThreadPoolExecutor(max_workers=max(1, multiprocessing.cpu_count() // 2)) as executor:
        future_to_poc_id = {executor.submit(run_poc, poc_id): poc_id for poc_id in POCS}
        for future in concurrent.futures.as_completed(future_to_poc_id):
            poc_id = future_to_poc_id[future]
            f = POCS[poc_id]
            result = future.result()
            if result != 'OK':
                all_good = False
            print("%s : %s" % (f, result))
    if all_good:
        sys.exit(0)
    else:
        sys.exit(-1)

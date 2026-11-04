#!/bin/sh
# Build normally once, then exercise the installed WHO cases selected from --list.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
python3.13 - <<'PY'
import json
import math
import os
from pathlib import Path
import subprocess
import sys

workers = str(max(1, math.floor(float(os.environ.get("PM_JOB_CORES", "2")))))
os.environ.setdefault("CARGO_BUILD_JOBS", workers)
os.environ.setdefault("RUST_TEST_THREADS", workers)
built = subprocess.check_output([
    'cargo', 'build', '--locked', '-p', 'biomcp', '--bin', 'biomcp',
    '--test', 'cases', '--message-format=json',
], text=True)
artifacts = [json.loads(line) for line in built.splitlines() if line.startswith('{')]
executables = {item['target']['name']: item['executable'] for item in artifacts
               if item.get('reason') == 'compiler-artifact' and item.get('executable')}
runner = str(Path(executables['cases']).resolve())
executable = str(Path(executables['biomcp']).resolve())
listed = subprocess.check_output([runner, '--list'], cwd='crates/biomcp', text=True)
names = [line.removesuffix(': test') for line in listed.splitlines()
         if line.endswith(('/who-dates.yaml: test', '/who-sync.yaml: test'))]
if len(names) != 2:
    raise SystemExit('WHO cases missing from --list: ' + repr(names))
print('WHO case selectors:', names, flush=True)
subprocess.run([sys.executable, 'scripts/trials.py', '--installed',
                '--runner', runner, '--executable', executable, *names], check=True)
PY

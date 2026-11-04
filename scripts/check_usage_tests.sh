#!/bin/sh
# Build once and run installed grammar cases selected from the runner's discovery.
set -eu
cd "$(dirname "$0")/.."
make check-cheap
python3 - <<'PY'
import os
from pathlib import Path
import shutil
import subprocess
from release.artifacts import cargo
binary = cargo('dev')['executable']
runner = cargo('dev', tests='cases')['executable']
stage = Path('target/usage-installed').resolve()
stage.mkdir(parents=True, exist_ok=True)
shutil.copy2(binary, stage / 'biomcp')
listed = subprocess.check_output([runner, '--list'], text=True, cwd='crates/biomcp')
(stage / 'cases-list.txt').write_text(listed)
print(listed, end='')
names = [line.removesuffix(': test') for line in listed.splitlines()
         if line.startswith('grammar/') and line.endswith(': test')]
if not names:
    raise SystemExit('No grammar cases discovered')
subprocess.run(['python3', 'scripts/trials.py', '--runner', runner,
                '--executable', str(stage / 'biomcp'), '--installed', *names], check=True)
PY

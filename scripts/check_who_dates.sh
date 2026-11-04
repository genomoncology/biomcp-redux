#!/bin/sh
# Build normally once, then exercise the installed WHO cases selected from --list.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
proof_python_bin=$(python3.13 -c 'from pathlib import Path; import sys; print(Path(sys.executable).resolve().parent)')
export PATH="$proof_python_bin:$PATH"
case "${1:-focused}" in
cheap) exec make check-cheap ;;
generate) exec make schema-generate docs-generate ;;
structure)
    export CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2
    exec cargo test --locked -p biomcp --test ratchets repository_obeys_structural_contract -- --exact
    ;;
scans)
    python3.13 - <<'SCAN'
import json
from pathlib import Path
import subprocess

scanner = next((package.parent.parent / '.bin/jscpd'
                for package in (Path.home() / '.npm/_npx').glob('*/node_modules/jscpd/package.json')
                if json.loads(package.read_text())['version'] == '4.0.5'), None)
if scanner is None:
    raise SystemExit('cached jscpd 4.0.5 is required')
files = ['crates/biomcp-adapters/src/who_pq/mod.rs',
         'crates/biomcp-adapters/src/who_pq/raw.rs',
         'crates/biomcp-query/src/drug/who.rs',
         'crates/biomcp-types/src/drug/query.rs',
         'crates/biomcp-types/src/response/error.rs']
subprocess.run([str(scanner), '--version'], check=True)
root = Path('target/who-date-scans')
for side in ['baseline', 'current']:
    paths = files
    if side == 'baseline':
        paths = []
        for name in files:
            path = root / 'baseline' / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(subprocess.check_output(['git', 'show', '220d8453a:' + name]))
            paths.append(str(path))
    subprocess.run([str(scanner), *paths, '--format', 'rust', '--min-tokens', '50',
                    '--max-size', '10mb', '--reporters', 'console,json',
                    '--output', str(root / (side + '-report'))], check=True)
SCAN
    exit ;;
focused) ;;
*) echo 'usage: check_who_dates.sh [focused|cheap|scans|structure|generate]' >&2; exit 2 ;;
esac
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

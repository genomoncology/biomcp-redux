#!/bin/sh
# Capture Windows live-check rows or check portable case names.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  windows-list)
    python3.13 - "$2" "$3" <<'PYTHON'
import base64
import subprocess
import sys

runner, fixtures = (value.replace("'", "''") for value in sys.argv[1:])
command = rf"""
$ErrorActionPreference = 'Stop'
$env:BIOMCP_TEST_FIXTURES = '{fixtures}'
$report = & '{runner}' --live-check --list
if ($LASTEXITCODE -ne 0) {{ throw 'live-check listing failed' }}
($report | ConvertFrom-Json).cases | Where-Object {{
    $_.case -match 'article[\\/]sections[\\/](grammar|failure-500)\.yaml$'
}} | ConvertTo-Json -Compress
"""
encoded = base64.b64encode(command.encode('utf-16le')).decode()
subprocess.run(['ssh', '-o', 'BatchMode=yes', '-o', 'ConnectTimeout=15', '-p', '2222',
                'Docker@localhost', 'powershell -NoProfile -EncodedCommand ' + encoded], check=True)
PYTHON
    ;;
  focused)
    make fmt lint-tools ratchets
    cargo test --locked -p biomcp --test tools -- --proofs live-source-checks/offline-eligibility --exact
    ;;
  *) exit 2 ;;
esac

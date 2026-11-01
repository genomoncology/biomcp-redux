#!/bin/sh
# Diagnose Windows inventory or check portable live-check case names.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  windows-inventory)
    ssh -p 2222 Docker@localhost 'powershell -NoProfile -Command "Get-ChildItem C:\ -Directory | Select-Object -ExpandProperty FullName; Get-ChildItem C:\Users\Docker -Directory | Select-Object -ExpandProperty FullName"'
    ;;
  focused)
    make fmt lint-tools ratchets
    cargo test --locked -p biomcp --test tools -- --proofs live-source-checks/offline-eligibility --exact
    ;;
  *) exit 2 ;;
esac

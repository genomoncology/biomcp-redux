#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
export PATH="$HOME/.local/bin:$PATH"
python3.13 --version
case "${1:-fixtures}" in
  fixtures)
    ruff check --select F401,F841,ERA,BLE,E722,S110 release/gate.py release/smoke.py release/test_gate.py scripts/trials.py
    python3.13 scripts/check_shell.py
    python3.13 -m unittest release.test_gate release.test_checks
    python3.13 scripts/test_trials.py
    ;;
  proof)
    export BIOMCP_TEST_PYTHON=$(python3.13 -c 'import sys; print(sys.executable)')
    cargo test --locked -p biomcp --test tools -- --proofs --exact live-source-checks/retained-drift-witnesses
    cargo clippy --locked -p biomcp --test tools -- -D warnings
    ;;
  *) exit 2 ;;
esac

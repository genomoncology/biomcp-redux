#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
gate_python_bin=$(python3.13 -c 'from pathlib import Path; import sys; print(Path(sys.executable).resolve().parent)')
export PATH="$gate_python_bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
export CARGO_BUILD_JOBS=2
if [ "${1:-check}" = partial ]; then
    exec cargo run --locked -p biomcp --example usage_partial_matches
fi
cargo build --locked -p biomcp --bin biomcp
binary="${CARGO_TARGET_DIR:-target}/debug/biomcp"
python3 scripts/test_argument_recovery.py "$binary" "${1:-check}"
if [ "${1:-check}" = focused ]; then
    cargo test --locked -p biomcp --test cases --no-run --message-format=json > "${CARGO_TARGET_DIR:-target}/argument-runner.jsonl"
    runner=$(python3 -c 'import json,sys; print(next(r["executable"] for s in open(sys.argv[1]) if (r:=json.loads(s)).get("executable") and r["target"]["name"]=="cases"))' "${CARGO_TARGET_DIR:-target}/argument-runner.jsonl")
    python3 scripts/trials.py --runner "$runner" --executable "$binary" --installed \
        --case-file grammar/source-filter-errors.yaml \
        --case-file grammar/usage-bad-enum.yaml \
        --case-file grammar/usage-bad-typed-id.yaml \
        --case-file grammar/usage-missing-option.yaml \
        --case-file grammar/usage-unknown-option.yaml \
        --case-file pgx/retired.yaml \
        --case-file local/refusals-get.yaml \
        --case-file local/refusals-search.yaml
fi

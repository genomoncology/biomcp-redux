#!/bin/sh
# Exercise FAERS reaction counts at the caller boundary.
set -eu
cd "$(dirname "$0")/.."
faers_python_bin=$(python3.13 -c 'from pathlib import Path; import sys; print(Path(sys.executable).resolve().parent)')
export PATH="$faers_python_bin:$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  generate)
    cargo fmt --all
    make schema-generate docs-generate
    ;;
  focused)
    cargo test --locked -p biomcp-types --features schema --test adverse_event_contracts
    python3.13 scripts/trials.py --case-file adverse-event/faers-reaction-counts.yaml --case-file adverse-event/examples.yaml --case-file adverse-event/faers-several-names.yaml --case-file adverse-event/help.yaml
    PYTHONPATH=. python3.13 scripts/faers_counts_mcp.py "${CARGO_TARGET_DIR:-target}/debug/biomcp"
    ;;
  scans)
    baseline=${2:?scans requires the current main baseline}
    uvx --offline --from ruff==0.17.2 ruff check --select F401,F841,ERA,BLE,E722,S110 scripts/faers_counts_mcp.py
    uvx --offline --from vulture==2.14 vulture scripts/faers_counts_mcp.py --min-confidence 60
    mkdir -p target/0662-scans/baseline
    git archive "$baseline" crates/biomcp-types/src/adverse_event crates/biomcp-query/src/adverse_event.rs crates/biomcp-query/src/adverse_event crates/biomcp-adapters/src/faers | tar -xf - -C target/0662-scans/baseline
    for side in baseline current; do
      prefix=.
      test "$side" != baseline || prefix=target/0662-scans/baseline
      npm exec --offline --yes --package=jscpd@4.0.5 -- jscpd "$prefix/crates/biomcp-types/src/adverse_event" "$prefix/crates/biomcp-query/src/adverse_event.rs" "$prefix/crates/biomcp-query/src/adverse_event" "$prefix/crates/biomcp-adapters/src/faers" --format rust --max-size 10mb --reporters console,json --output "target/0662-scans/$side"
    done
    ;;
  routine)
    sh scripts/faers_counts_job.sh scans "${2:?routine requires the current main baseline}"
    make landing-gate
    ;;
  *) exit 2 ;;
esac

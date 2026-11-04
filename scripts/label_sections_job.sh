#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
case "${1:-focused}" in
  generate) cargo fmt --all; make schema-generate docs-generate ;;
  list) python3.13 scripts/trials.py --target cases --list ;;
  focused)
    make check-cheap
    shift
    runner=${1:?focused requires discovered runner}; shift
    python3.13 scripts/trials.py --target cases --runner "$runner" --executable "${CARGO_TARGET_DIR:-target}/debug/biomcp" --installed "$@"
    ;;
  scans)
    baseline=${2:?scans requires baseline commit}
    mkdir -p target/0608-scans/baseline
    git archive "$baseline" crates/biomcp-types/src/drug crates/biomcp-adapters/src/openfda_label crates/biomcp-query/src/drug | tar -xf - -C target/0608-scans/baseline
    for side in baseline current; do
      prefix=.
      test "$side" != baseline || prefix=target/0608-scans/baseline
      npm exec --offline --yes --package=jscpd@4.0.5 -- jscpd "$prefix/crates/biomcp-types/src/drug" "$prefix/crates/biomcp-adapters/src/openfda_label" "$prefix/crates/biomcp-query/src/drug" --format rust --max-size 10mb --reporters json --output "target/0608-scans/$side"
    done
    ;;
  routine) make landing-gate ;;
  *) exit 2 ;;
esac

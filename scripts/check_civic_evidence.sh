#!/bin/sh
# Exercise evidence filters against native request recordings through the installed CLI.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  focused)
    output="${CARGO_TARGET_DIR:-target}/civic-evidence"
    mkdir -p "$output"
    make -s --no-print-directory civic-evidence-build > "$output/build.jsonl"
    runner=$(python3 -c 'import json,sys; rows=[json.loads(line) for line in open(sys.argv[1]) if line.startswith("{")]; print(next(row["executable"] for row in rows if row.get("reason")=="compiler-artifact" and row["target"]["name"]=="cases" and row.get("executable")))' "$output/build.jsonl")
    python3 scripts/trials.py --runner "$runner" --executable "${CARGO_TARGET_DIR:-target}/debug/biomcp" --list > "$output/list.txt"
    rg 'civic-evidence' "$output/list.txt"
    python3 scripts/trials.py --runner "$runner" --executable "${CARGO_TARGET_DIR:-target}/debug/biomcp" --installed \
      drug-civic/drug/civic-evidence-public.yaml drug-civic/drug/civic-evidence-streams.yaml
    ;;
  generate)
    make schema-generate docs-generate
    mkdir -p target/civic-evidence
    git diff --binary > target/civic-evidence/generated.patch
    ;;
  cheap) make check-cheap ;;
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  *) echo 'usage: check_civic_evidence.sh [focused|generate|cheap|routine]' >&2; exit 2 ;;
esac

#!/bin/sh
# Compare a bounded selection with the actual released baseline; print one summary.
set -eu
plan=${1:?PLAN must name a parity selection}
baseline=${2:?BASELINE must name the released 0.9.3 executable}
out=${3:?OUT must name a new evidence directory}
cargo test --locked -p biomcp --test tools -- --parity "$plan" "$baseline" "$out"

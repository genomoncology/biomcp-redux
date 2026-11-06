#!/bin/sh
# Exercise publisher dates through the installed CLI with recorded replies.
set -eu
case "${1:-focused}" in
  focused)
    export BIOMCP_TRANSCRIPT_DIR="${CARGO_TARGET_DIR:-target}/source-age-proof/transcripts"
    cargo test --locked -p biomcp-fetch calendar_days_preserve_leap_years_and_reject_partial_dates
    sh scripts/check_study_rows.sh focused --case-file source-age/openfda.yaml --case-file source-age/configured.yaml --case-file source-age/ema.yaml --case-file source-age/cellosaurus.yaml --case-file source-age/chembl.yaml --case-file source-age/ctgov.yaml
    ;;
  prepare)
    cargo fmt --all
    cargo check --locked -p biomcp
    ;;
  cheap)
    make check-cheap
    make docs-check
    ;;
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  *) exit 2 ;;
esac

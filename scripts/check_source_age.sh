#!/bin/sh
# Exercise publisher dates through the installed CLI with recorded replies.
set -eu
case "${1:-focused}" in
  focused)
    source_age_proof=$(mktemp -d "${TMPDIR:-/tmp}/source-age-proof.XXXXXX")
    export BIOMCP_TRANSCRIPT_DIR="$source_age_proof/transcripts"
    set +e
    sh "$0" cases > "$source_age_proof/run.log" 2>&1
    source_age_status=$?
    set -e
    cat "$source_age_proof/run.log"
    source_age_revision=$(git rev-parse HEAD)
    git rev-parse HEAD > "$source_age_proof/source.txt"
    COPYFILE_DISABLE=1 tar -C "$source_age_proof" -czf "$source_age_proof.tar.gz" .
    ssh -o BatchMode=yes -o ConnectTimeout=15 yellow 'mkdir -p /home/ian/workspace/data/biomcp-redux-evidence/0718'
    scp -o BatchMode=yes -o ConnectTimeout=15 "$source_age_proof.tar.gz" "yellow:/home/ian/workspace/data/biomcp-redux-evidence/0718/source-age-$source_age_revision.tar.gz"
    rm -rf "$source_age_proof" "$source_age_proof.tar.gz"
    exit "$source_age_status"
    ;;
  cases)
    cargo test --locked -p biomcp-fetch calendar_days_preserve_leap_years_and_reject_partial_dates
    cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    sh scripts/check_study_rows.sh focused --case-file source-age/comparison.yaml --case-file source-age/fda-unavailable.yaml --case-file drug/ema.yaml --case-file drug/ema-sync.yaml --case-file source-age/openfda.yaml --case-file source-age/configured.yaml --case-file source-age/ema.yaml --case-file source-age/cellosaurus.yaml --case-file source-age/chembl.yaml --case-file source-age/ctgov.yaml source-data-age/restored-age source-data-age/hosted-stale-ema
    ;;
  corrections)
    sh scripts/check_study_rows.sh focused --case-file source-age/fda-unavailable.yaml --case-file source-age/ema.yaml
    ;;
  prepare)
    cargo fmt --all
    cargo check --locked -p biomcp
    make schema-generate
    source_age_revision=$(git rev-parse HEAD)
    mkdir -p "${CARGO_TARGET_DIR:-target}"
    git diff --binary > "${CARGO_TARGET_DIR:-target}/source-age-generated.patch"
    ssh -o BatchMode=yes yellow 'mkdir -p /home/ian/workspace/data/biomcp-redux-evidence/0718'
    scp "${CARGO_TARGET_DIR:-target}/source-age-generated.patch" "yellow:/home/ian/workspace/data/biomcp-redux-evidence/0718/generated-$source_age_revision.patch"
    ;;
  cheap)
    make check-cheap
    make docs-check
    ;;
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  *) exit 2 ;;
esac

#!/bin/sh
# Follow cargo-llvm-cov's documented external-test sequence.
set -eu
cd "$(dirname "$0")/.."

# One product-source denominator for both collections, including child processes.
ignore='(^|/)(tests|target|generated)/|(^|/)build\.rs$|_tests\.rs$'
threshold=$(awk '
    NR != 1 || $0 !~ /^[0-9]+([.][0-9]+)?$/ || $0 + 0 > 100 { bad = 1 }
    END {
        if (NR != 1 || bad) exit 1
        floor = $0 - 0.2
        printf "%.4f\n", (floor > 0 ? floor : 0)
    }
' coverage.floor) || { echo 'coverage: invalid coverage.floor' >&2; exit 1; }

coverage_env=$(cargo llvm-cov show-env --sh)
eval "$coverage_env"
profile_dir=${LLVM_PROFILE_FILE%/*}
case "$profile_dir" in
    /*) ;;
    *) echo 'coverage: expected an absolute profile directory' >&2; exit 1 ;;
esac
complete="$profile_dir/coverage-complete"
summary=$(mktemp)
saved=$(mktemp -d)
cleanup() {
    # Library profiles never remain available to a later cases-only report.
    if [ -f "$saved/collecting-library" ]; then
        rm -f "$profile_dir"/*.profraw
        for profile in "$saved"/*.profraw; do
            [ ! -f "$profile" ] || mv "$profile" "$profile_dir/"
        done
    fi
    rm -rf "$saved"
    rm -f "$summary"
}
trap cleanup EXIT HUP INT TERM

report() {
    label=$1
    shift
    report_result=0
    cargo llvm-cov report --locked --ignore-filename-regex "$ignore" "$@" >"$summary" || report_result=$?
    awk -v label="$label" '/^TOTAL/ { print "coverage (" label "; product source): " $0 }' "$summary"
    return "$report_result"
}

case "${1:-run}" in
    run)
        cargo llvm-cov clean --workspace
        mkdir -p "$profile_dir"
        rm -f "$complete"
        # Build both collections before reporting, so neither gains a smaller
        # denominator from product functions linked only by the other collection.
        cargo test --workspace --locked --no-run
        rm -f "$profile_dir"/*.profraw
        LLVM_PROFILE_FILE="$profile_dir/cases-%m-%p.profraw"
        export LLVM_PROFILE_FILE
        if ! cargo test --locked -p biomcp --test cases; then
            echo 'coverage: cases collection incomplete because its suite failed' >&2
            exit 1
        fi
        result=0
        report 'outside-in cases' --fail-under-lines "$threshold" || result=$?
        # Save only cases profiles outside the directory cargo-llvm-cov scans.
        for profile in "$profile_dir"/*.profraw; do
            [ ! -f "$profile" ] || mv "$profile" "$saved/"
        done
        touch "$saved/collecting-library"
        LLVM_PROFILE_FILE="$profile_dir/library-%m-%p.profraw"
        export LLVM_PROFILE_FILE
        if ! cargo test --workspace --exclude biomcp --locked ||
           ! cargo test --locked -p biomcp --test ratchets; then
            echo 'coverage: library collection incomplete because its suite failed' >&2
            exit 1
        fi
        report 'permitted library tests; excluded from floor'
        # Restore cases profiles even when their floor check failed.
        touch "$complete"
        exit "$result"
        ;;
    report|unreached)
        if [ ! -f "$complete" ]; then
            echo 'coverage: no complete collection; run make coverage first' >&2
            exit 1
        fi
        if [ "$1" = unreached ]; then
            cargo llvm-cov report --locked --ignore-filename-regex "$ignore" --show-missing-lines
        else
            report 'outside-in cases' --fail-under-lines "$threshold"
        fi
        ;;
    *) echo 'usage: coverage.sh [run|report|unreached]' >&2; exit 2 ;;
esac

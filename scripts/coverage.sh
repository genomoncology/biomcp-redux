#!/bin/sh
# Follow cargo-llvm-cov's documented external-test sequence.
set -eu
cd "$(dirname "$0")/.."

# Only product Rust source belongs in the denominator. Cargo also excludes
# dependencies and toolchain sources by default.
ignore='(^|/)(tests|target|generated)/|(^|/)build\.rs$'
threshold=$(awk '
    NR != 1 || $0 !~ /^[0-9]+([.][0-9]+)?$/ || $0 + 0 > 100 { bad = 1 }
    END {
        if (NR != 1 || bad) exit 1
        floor = $0 - 0.2
        printf "%.4f\n", floor > 0 ? floor : 0
    }
' coverage.floor) || { echo 'coverage: invalid coverage.floor' >&2; exit 1; }

case "${1:-run}" in
    run)
        coverage_env=$(cargo llvm-cov show-env --sh)
        eval "$coverage_env"
        cargo llvm-cov clean --workspace
        profile_dir=${LLVM_PROFILE_FILE%/*}
        case "$profile_dir" in
            /*) ;;
            *) echo 'coverage: expected an absolute profile directory' >&2; exit 1 ;;
        esac
        mkdir -p "$profile_dir"
        LLVM_PROFILE_FILE="$profile_dir/coverage-%m-%p.profraw"
        export LLVM_PROFILE_FILE
        if ! cargo test --workspace --locked; then
            echo 'coverage: collection incomplete because the owning suite failed' >&2
            exit 1
        fi
        ;;
    report) ;;
    unreached)
        exec cargo llvm-cov report --locked --ignore-filename-regex "$ignore" --show-missing-lines
        ;;
    *) echo 'usage: coverage.sh [run|report|unreached]' >&2; exit 2 ;;
esac

summary=$(mktemp)
trap 'rm -f "$summary"' EXIT HUP INT TERM
result=0
cargo llvm-cov report --locked --ignore-filename-regex "$ignore" \
    --fail-under-lines "$threshold" >"$summary" || result=$?
awk '/^TOTAL/ { print "coverage (product source): " $0 }' "$summary"
exit "$result"

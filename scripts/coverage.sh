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
    # Excluded profiles never remain available to a later outside-in report.
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

# Bind reusable profiles to the inputs that produced them, including dirty and
# untracked source files. Git's revision alone misses working-tree changes.
inputs() {
    python3 - <<'PYINPUTS'
import hashlib
from pathlib import Path
import subprocess

names = subprocess.check_output([
    'git', 'ls-files', '-z', '--cached', '--others', '--exclude-standard', '--',
    'crates', 'tests', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml',
    'scripts/coverage.sh', 'scripts/contract_targets.py', 'sdlc/features.md',
]).split(b'\0')
digest = hashlib.sha256()
for name in sorted(set(names) - {b''}):
    path = Path(name.decode())
    digest.update(name + b'\0')
    digest.update(path.read_bytes() if path.is_file() else b'<missing>')
    digest.update(b'\0')
print(digest.hexdigest())
PYINPUTS
}

report() {
    label=$1
    shift
    report_result=0
    cargo llvm-cov report --locked --ignore-filename-regex "$ignore" "$@" >"$summary" || report_result=$?
    awk -v label="$label" '/^TOTAL/ { print "coverage (" label "; product source): " $0 }' "$summary"
    counts=$(awk '/^TOTAL/ { print $8, $9 }' "$summary")
    return "$report_result"
}

case "${1:-run}" in
    run)
        cargo llvm-cov clean --workspace
        mkdir -p "$profile_dir"
        rm -f "$complete"
        collected_inputs=$(inputs)
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
        # Keep the historical command-only figure on the same denominator.
        report 'command cases only; historical comparison'
        contracts=$(python3 scripts/contract_targets.py)
        printf '%s\n' "$contracts" | while IFS="$(printf '\t')" read -r package target features owner; do
            [ -n "$package" ] || continue
            echo "coverage: counted public contract $package/$target; feature $owner; cargo features $features"
            set --
            if [ "$features" != "-" ]; then set -- --features "$features"; fi
            LLVM_PROFILE_FILE="$profile_dir/contracts-%m-%p.profraw" \
                cargo test --locked -p "$package" --test "$target" "$@" || {
                echo 'coverage: public contract collection incomplete because its suite failed' >&2
                exit 1
            }
        done
        result=0
        report 'command cases plus reviewed public contracts' --fail-under-lines "$threshold" || result=$?
        cases_counts=$counts
        # Save only counted outside-in profiles outside the directory cargo-llvm-cov scans.
        for profile in "$profile_dir"/*.profraw; do
            [ ! -f "$profile" ] || mv "$profile" "$saved/"
        done
        touch "$saved/collecting-library"
        LLVM_PROFILE_FILE="$profile_dir/library-%m-%p.profraw"
        export LLVM_PROFILE_FILE
        if ! python3 scripts/contract_targets.py --library ||
           ! cargo test --locked -p biomcp --test ratchets; then
            echo 'coverage: library collection incomplete because its suite failed' >&2
            exit 1
        fi
        report 'unlisted contracts and library tests; excluded from floor'
        library_counts=$counts
        # The union measures overlap only. It never supplies the floor check.
        for profile in "$saved"/*.profraw; do
            [ ! -f "$profile" ] || cp "$profile" "$profile_dir/"
        done
        cargo llvm-cov report --locked --ignore-filename-regex "$ignore" >"$summary"
        union_counts=$(awk '/^TOTAL/ { print $8, $9 }' "$summary")
        awk -v cases="$cases_counts" -v library="$library_counts" -v union="$union_counts" '
            BEGIN {
                split(cases, a); split(library, b); split(union, u)
                if (a[1] != b[1] || a[1] != u[1]) {
                    print "coverage: collection denominators differ" > "/dev/stderr"
                    exit 1
                }
                printf "coverage lines: outside only %d; library only %d; both %d; unobserved %d; incomplete collections 0\n", b[2]-u[2], a[2]-u[2], u[1]-a[2]-b[2]+u[2], u[2]
            }
        '
        # Restore counted profiles even when their floor check failed.
        if [ "$(inputs)" != "$collected_inputs" ]; then
            echo 'coverage: inputs changed during collection; run make coverage again' >&2
            exit 1
        fi
        printf '%s\n' "$collected_inputs" >"$complete"
        exit "$result"
        ;;
    report|unreached)
        if [ ! -f "$complete" ]; then
            echo 'coverage: no complete collection; run make coverage first' >&2
            exit 1
        fi
        if [ "$(cat "$complete")" != "$(inputs)" ]; then
            echo 'coverage: inputs changed since collection; run make coverage again' >&2
            exit 1
        fi
        if [ "$1" = unreached ]; then
            cargo llvm-cov report --locked --ignore-filename-regex "$ignore" --show-missing-lines
        else
            report 'command cases plus reviewed public contracts' --fail-under-lines "$threshold"
        fi
        ;;
    *) echo 'usage: coverage.sh [run|report|unreached]' >&2; exit 2 ;;
esac

#!/bin/sh
set -eu
case "${1:-focused}" in
red) python3 -m unittest release.test_security.Screening.test_history_exceptions_require_the_exact_blob_rule_line_and_commit ;;
cheap) make check-cheap ;;
scans)
    uvx --offline --from ruff==0.17.2 ruff --version
    uvx --offline --from ruff==0.17.2 ruff check --select F401,F841,ERA,BLE,E722,S110 release/secrets.py release/test_checks.py release/test_security.py
    uvx --offline --from vulture==2.14 vulture --version
    uvx --offline --from vulture==2.14 vulture release/secrets.py --min-confidence 60
    npm exec --offline --yes --package=jscpd@4.0.5 -- jscpd --version
    npm exec --offline --yes --package=jscpd@4.0.5 -- jscpd release/secrets.py release/test_checks.py release/test_security.py --format python --max-size 10mb --reporters json --output target/0638-scans
    ;;
focused) python3 -m unittest release.test_security scripts.test_gate_stages ;;
main-only)
    scratch=$(mktemp -d)
    trap 'rm -rf "$scratch"' EXIT HUP INT TERM
    git init --quiet "$scratch/repo"
    git -C "$scratch/repo" fetch --quiet --no-tags "$(pwd)" HEAD:refs/heads/main
    git -C "$scratch/repo" checkout --quiet main
    python3 "$scratch/repo/release/secrets.py" history "$scratch/repo" --cache "$(pwd)/target/security-tools"
    ;;
*) exit 2 ;;
esac

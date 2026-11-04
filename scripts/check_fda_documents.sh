#!/bin/sh
# Build the native FDA document/search changes once, then run exact installed cases.
set -eu
cd "$(dirname "$0")/.."
case "${1:-focused}" in
  prepare)
    make schema-generate
    make docs-generate
    exec make check-cheap ;;
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  focused)
    make schema-generate
    make docs-generate
    make check-cheap
    python3 scripts/trials.py --target cases --list > target/fda-documents-trials.txt
    python3 scripts/trials.py --target cases --installed diagnostic/device-approvals diagnostic/fda-documents diagnostic/pdf-over-limit diagnostic/pdf-scanned-no-text diagnostic/pma diagnostic/help
    ;;
  *) echo 'usage: check_fda_documents.sh [prepare|focused|routine]' >&2; exit 2 ;;
esac

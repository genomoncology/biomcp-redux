#!/bin/sh
# Build the native FDA document/search changes once, then run exact installed cases.
set -eu
cd "$(dirname "$0")/.."
case "${1:-cases}" in
  prepare)
    make schema-generate
    make docs-generate
    exec make check-cheap ;;
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  live)
    make plain-cli-build
    exec python3 scripts/check_fda_document_live.py ;;
  cases)
    make schema-generate
    make docs-generate
    make plain-cli-build
    python3 scripts/trials.py --target cases --list > target/fda-documents-trials.txt
    python3 scripts/trials.py --target cases --installed device-approval-search/diagnostic/device-approvals.yaml fda-document-text/diagnostic/fda-documents.yaml fda-document-text/diagnostic/pdf-over-limit.yaml fda-document-text/diagnostic/pdf-scanned-no-text.yaml diagnostic-pma/diagnostic/pma.yaml diagnostic-cdx/diagnostic/help.yaml
    ;;
  *) echo 'usage: check_fda_documents.sh [prepare|cases|routine|live]' >&2; exit 2 ;;
esac

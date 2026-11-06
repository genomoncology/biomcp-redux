#!/bin/sh
# Keep exact focused runs and synthetic inputs outside disposable build machines.
set -eu
cd "$(dirname "$0")/.."
proof=target/article-rights-proof
mkdir -p "$proof"
export BIOMCP_EVIDENCE_DIR="$PWD/$proof"
retain() {
    status=$?
    trap - EXIT
    cat "$proof/run.log"
    sha=$(git rev-parse HEAD)
    run=$(date -u +%Y%m%dT%H%M%SZ)
    destination="/home/ian/workspace/data/biomcp-redux-evidence/0689/$sha/$run"
    archive=target/article-rights-proof.tar.gz
    tar -czf "$archive" -C "$proof" .
    ssh yellow mkdir -p "$destination"
    scp "$archive" "yellow:$destination/proof.tar.gz"
    printf 'Evidence: yellow:%s/proof.tar.gz\n' "$destination"
    exit "$status"
}
trap retain EXIT
case "${1:-focused}" in
    focused)
        cargo test --locked -p biomcp-query --test article_rights -- --nocapture > "$proof/run.log" 2>&1
        ;;
    authors)
        cargo test --locked -p biomcp-query --test article_rights authors:: -- --nocapture > "$proof/run.log" 2>&1
        ;;
    prepare)
        cargo fmt --all > "$proof/run.log" 2>&1
        UPDATE_SCHEMAS=1 cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact >> "$proof/run.log" 2>&1
        make docs-generate >> "$proof/run.log" 2>&1
        git diff --binary > "$proof/generated.patch"
        ;;
    *) printf 'unknown article rights check\n' >&2; exit 2 ;;
esac

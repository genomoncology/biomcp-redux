#!/bin/sh
# Keep the exact focused run and synthetic inputs outside the disposable build machine.
set -eu
cd "$(dirname "$0")/.."
proof=target/article-rights-proof
mkdir -p "$proof"
export BIOMCP_EVIDENCE_DIR="$PWD/$proof"
status=0
cargo test --locked -p biomcp-query --test article_rights -- --nocapture > "$proof/focused.log" 2>&1 || status=$?
cat "$proof/focused.log"
sha=$(git rev-parse HEAD)
run=$(date -u +%Y%m%dT%H%M%SZ)
destination="/home/ian/workspace/data/biomcp-redux-evidence/0689/$sha/$run"
archive=target/article-rights-proof.tar.gz
tar -czf "$archive" -C "$proof" .
ssh yellow mkdir -p "$destination"
scp "$archive" "yellow:$destination/proof.tar.gz"
printf 'Evidence: yellow:%s/proof.tar.gz\n' "$destination"
exit "$status"

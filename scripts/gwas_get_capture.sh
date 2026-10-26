#!/bin/sh
set -eu
mkdir -p "$HOME/job-evidence"
capture_parent=$(mktemp -d "$HOME/job-evidence/biomcp-redux-0218.XXXXXX")
capture_status=0
python3 scripts/source_capture.py --plan scripts/gwas_get_capture.json --out "$capture_parent/native" || capture_status=$?
ssh yellow 'mkdir -p /home/ian/workspace/data/biomcp-redux/0218-gwas-get'
verify_dir=$(ssh yellow 'mktemp -d /tmp/0218-verify.XXXXXX')
scp "$capture_parent/native.tar.gz" yellow:/home/ian/workspace/data/biomcp-redux/0218-gwas-get/native.tar.gz
capture_hash=$(shasum -a 256 "$capture_parent/native.tar.gz" | cut -d ' ' -f 1)
ssh yellow "printf '%s  %s\n' '$capture_hash' /home/ian/workspace/data/biomcp-redux/0218-gwas-get/native.tar.gz | sha256sum -c -"
scp scripts/source_capture.py scripts/capture_outcome.py yellow:"$verify_dir/"
ssh yellow "TMPDIR=/tmp python3 $verify_dir/source_capture.py --verify /home/ian/workspace/data/biomcp-redux/0218-gwas-get/native.tar.gz"
ssh yellow "rm -rf $verify_dir"
rm -rf "$capture_parent"
exit "$capture_status"

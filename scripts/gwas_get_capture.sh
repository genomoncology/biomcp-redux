#!/bin/sh
set -eu
mkdir -p "$HOME/job-evidence"
capture_parent=$(mktemp -d "$HOME/job-evidence/biomcp-redux-0218.XXXXXX")
python3 scripts/source_capture.py --plan scripts/gwas_get_capture.json --out "$capture_parent/native"
ssh yellow 'mkdir -p /home/ian/workspace/data/biomcp-redux/0218-gwas-get'
ssh yellow "mkdir -p $capture_parent"
scp "$capture_parent/native.tar.gz" yellow:/home/ian/workspace/data/biomcp-redux/0218-gwas-get/native.tar.gz
capture_hash=$(shasum -a 256 "$capture_parent/native.tar.gz" | cut -d ' ' -f 1)
ssh yellow "printf '%s  %s\n' '$capture_hash' /home/ian/workspace/data/biomcp-redux/0218-gwas-get/native.tar.gz | sha256sum -c -"
scp scripts/source_capture.py scripts/capture_outcome.py yellow:"$capture_parent/"
ssh yellow "TMPDIR=/tmp python3 $capture_parent/source_capture.py --verify /home/ian/workspace/data/biomcp-redux/0218-gwas-get/native.tar.gz"
ssh yellow "rm -rf $capture_parent"
rm -rf "$capture_parent"

#!/bin/sh
set -eu
capture_parent=$(mktemp -d /tmp/biomcp-0234-capture.XXXXXX)
python3 scripts/source_capture.py --plan scripts/gwas_search_capture.json --out "$capture_parent/native"
ssh yellow 'mkdir -p /home/ian/workspace/data/biomcp-redux/0234-gwas-search'
scp "$capture_parent/native.tar.gz" yellow:/home/ian/workspace/data/biomcp-redux/0234-gwas-search/native-evidence.tar.gz
ssh yellow "mkdir -p $capture_parent"
scp scripts/source_capture.py scripts/capture_outcome.py yellow:"$capture_parent/"
ssh yellow "TMPDIR=/tmp python3 $capture_parent/source_capture.py --verify /home/ian/workspace/data/biomcp-redux/0234-gwas-search/native-evidence.tar.gz"
ssh yellow "rm -rf $capture_parent"
rm -rf "$capture_parent"

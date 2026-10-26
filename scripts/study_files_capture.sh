#!/bin/sh
set -eu
mkdir -p "$HOME/job-evidence"
capture_parent=$(mktemp -d "$HOME/job-evidence/biomcp-redux-0221.XXXXXX")
python3 scripts/source_capture.py --plan scripts/study_files_capture.json --out "$capture_parent/native"
ssh yellow 'mkdir -p /home/ian/workspace/data/biomcp-redux/0221-study-files'
scp "$capture_parent/native.tar.gz" yellow:/home/ian/workspace/data/biomcp-redux/0221-study-files/native.tar.gz
capture_hash=$(shasum -a 256 "$capture_parent/native.tar.gz" | cut -d ' ' -f 1)
ssh yellow "printf '%s  %s\n' '$capture_hash' /home/ian/workspace/data/biomcp-redux/0221-study-files/native.tar.gz | sha256sum -c -"
rm -rf "$capture_parent"

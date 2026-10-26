#!/bin/sh
set -eu
native_root=$(mktemp -d "${TMPDIR%/}/0234-native.XXXXXX")
trap 'rm -rf "$native_root"' EXIT
scp yellow:/home/ian/workspace/data/biomcp-redux/0205-gwas-captures/j14351-native-evidence.tar.gz "$native_root/previous.tar.gz"
scp yellow:/home/ian/workspace/data/biomcp-redux/0234-gwas-search/native-evidence.tar.gz "$native_root/current.tar.gz"
scp yellow:/home/ian/workspace/data/biomcp-redux/0193-gwas-investigation/2292-gwas-source-and-cost-evidence.tar.gz "$native_root/original.tar.gz"
python3 scripts/gwas_search_replay.py "$native_root" "$native_root/previous.tar.gz" "$native_root/current.tar.gz" "$native_root/original.tar.gz"
BIOMCP_TEST_FIXTURES="$native_root/fixtures" cargo test --locked -p biomcp --test cases -- gwas-search/retained-native-witnesses-and-costs

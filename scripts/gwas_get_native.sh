#!/bin/sh
set -eu
mkdir -p "$HOME/job-evidence/biomcp-redux-0218"
native_root=$(mktemp -d "$HOME/job-evidence/biomcp-redux-0218/native.XXXXXX")
scp yellow:/home/ian/workspace/data/biomcp-redux/0218-gwas-get/native.tar.gz "$native_root/input.tar.gz"
python3 scripts/gwas_get_replay.py "$native_root/fixtures" "$native_root/input.tar.gz"
BIOMCP_TEST_FIXTURES="$native_root/fixtures" cargo test --locked -p biomcp --test cases -- gwas-get/retained-native-records-and-costs
PYTHONDONTWRITEBYTECODE=1 python3 - "$native_root/fixtures/gwas-get-native/outputs" "$native_root/product.tar.gz" <<'PYARCHIVE'
import os
from pathlib import Path
import subprocess
import sys
sys.path.insert(0,str(Path('scripts').resolve()))
from source_capture import manifest, verify_archive
out=Path(sys.argv[1])
archive=Path(sys.argv[2])
manifest(out)
subprocess.run(['tar','-czf',str(archive),'-C',str(out),*sorted(path.name for path in out.iterdir())],check=True,env=dict(os.environ,COPYFILE_DISABLE='1'))
verify_archive(archive)
PYARCHIVE
proof_sha=$(git rev-parse HEAD)
evidence_path="/home/ian/workspace/data/biomcp-redux/0218-gwas-get/product-$proof_sha.tar.gz"
ssh yellow "test ! -e $evidence_path"
scp "$native_root/product.tar.gz" "yellow:$evidence_path"
verify_dir=$(ssh yellow 'mktemp -d /tmp/0218-product-verify.XXXXXX')
scp scripts/source_capture.py scripts/capture_outcome.py "yellow:$verify_dir/"
ssh yellow "TMPDIR=/tmp python3 $verify_dir/source_capture.py --verify $evidence_path"
ssh yellow "rm -rf $verify_dir"
shasum -a 256 "$native_root/product.tar.gz"
printf 'Native work: %s\n' "$native_root"

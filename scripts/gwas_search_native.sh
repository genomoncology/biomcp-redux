#!/bin/sh
set -eu
native_root=$(mktemp -d "${TMPDIR%/}/0234-native.XXXXXX")
trap 'rm -rf "$native_root"' EXIT
scp yellow:/home/ian/workspace/data/biomcp-redux/0205-gwas-captures/j14351-native-evidence.tar.gz "$native_root/previous.tar.gz"
scp yellow:/home/ian/workspace/data/biomcp-redux/0234-gwas-search/native-evidence.tar.gz "$native_root/current.tar.gz"
scp yellow:/home/ian/workspace/data/biomcp-redux/0193-gwas-investigation/2292-gwas-source-and-cost-evidence.tar.gz "$native_root/original.tar.gz"
python3 scripts/gwas_search_replay.py "$native_root" "$native_root/previous.tar.gz" "$native_root/current.tar.gz" "$native_root/original.tar.gz"
if [ "$#" -eq 1 ]; then
    mkdir "$native_root/selected"
    cp "$native_root/fixtures/gwas-native-cases/$1.yaml" "$native_root/selected/"
    BIOMCP_TEST_FIXTURES="$native_root/fixtures" cargo test --locked -p biomcp --test cases -- --cases "$native_root/selected"
else
    BIOMCP_TEST_FIXTURES="$native_root/fixtures" cargo test --locked -p biomcp --test cases -- gwas-search/retained-native-witnesses-and-costs
fi

if [ "$#" -eq 0 ]; then
    PYTHONDONTWRITEBYTECODE=1 python3 - "$native_root/fixtures/gwas-native-outputs" "$native_root/product.tar.gz" <<'PYARCHIVE'
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
    evidence_path="/home/ian/workspace/data/biomcp-redux/0234-gwas-search/product-$proof_sha.tar.gz"
    ssh yellow "test ! -e $evidence_path"
    scp "$native_root/product.tar.gz" "yellow:$evidence_path"
    verify_dir=$(ssh yellow 'mktemp -d /tmp/0234-product-verify.XXXXXX')
    scp scripts/source_capture.py scripts/capture_outcome.py "yellow:$verify_dir/"
    ssh yellow "TMPDIR=/tmp python3 $verify_dir/source_capture.py --verify $evidence_path"
    ssh yellow "python3 -c 'import shutil; shutil.rmtree("$verify_dir")'"
    shasum -a 256 "$native_root/product.tar.gz"
fi

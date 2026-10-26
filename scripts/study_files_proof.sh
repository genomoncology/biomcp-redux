#!/bin/bash
set -euo pipefail
revision=$(git rev-parse HEAD)
mkdir -p "$HOME/job-evidence/biomcp-redux-0221"
proof=$(mktemp -d "$HOME/job-evidence/biomcp-redux-0221/native.XXXXXX")
scp yellow:/home/ian/workspace/data/biomcp-redux/0221-study-files/native.tar.gz "$proof/native.tar.gz"
scp yellow:/home/ian/workspace/data/biomcp-redux/0194-study-investigation/j14206-native-metadata.tar.gz "$proof/catalog.tar.gz"
scp yellow:/home/ian/workspace/data/biomcp-redux/0207-study-bounded-captures/2293-study-admission.tar.gz "$proof/admission.tar.gz"
python3 scripts/study_files_retained.py "$proof/native.tar.gz" "$proof/catalog.tar.gz" "$proof/admission.tar.gz" "$proof/prepared"
set +e
BIOMCP_TEST_FIXTURES="$proof/prepared" cargo test -p biomcp --test cases -- study-files/ --nocapture 2>&1 | tee "$proof/product-proof.log"
result=${PIPESTATUS[0]}
set -e
printf '%s\n' "$revision" > "$proof/revision"
printf '%s\n' "$result" > "$proof/exit-status"
python3 - "$proof" <<'PY'
import hashlib,json,sys
from pathlib import Path
root=Path(sys.argv[1])
files={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in root.rglob('*') if p.is_file()}
(root/'manifest.json').write_text(json.dumps(files,sort_keys=True,indent=2)+'\n')
PY
COPYFILE_DISABLE=1 tar -czf "$proof.tar.gz" -C "$proof" .
scp "$proof.tar.gz" yellow:"/home/ian/workspace/data/biomcp-redux/0221-study-files/native-proof-$revision.tar.gz"
local_hash=$(shasum -a 256 "$proof.tar.gz" | cut -d ' ' -f 1)
remote_hash=$(ssh yellow "sha256sum /home/ian/workspace/data/biomcp-redux/0221-study-files/native-proof-$revision.tar.gz" | cut -d ' ' -f 1)
test "$local_hash" = "$remote_hash"
printf 'Retained proof archive SHA-256 %s\n' "$local_hash"
exit "$result"

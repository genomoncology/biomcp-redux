#!/bin/sh
# Use the qualified frozen ARM runtime, without rebuilding or observing live SQL.
set -eu
cd "$(dirname "$0")/.."
source_base=/home/ian/workspace/data/biomcp-redux-evidence/0293/candidate-015aea0a704b7/cold-arm64-j30271
root="$PWD/target/installed-cache-first-open"
mkdir -p "$root/artifact"
scp "yellow:$source_base/dist/biomcp-container-arm64.tar" "$root/artifact/runtime.tar"
scp "yellow:$source_base/dist/release.json" "$root/artifact/release.json"
scp "yellow:$source_base/proof/bootstrap.json" "$root/artifact/bootstrap.json"
python3 - "$root/artifact" <<'PY'
import hashlib
import json
from pathlib import Path
import sys
import tarfile
root = Path(sys.argv[1])
release = json.loads((root / 'release.json').read_text())
bootstrap = json.loads((root / 'bootstrap.json').read_text())
source = '015aea0a704b7b6176b6988bc6fae707ac0feb38'
assert release['revision'] == source
assert bootstrap['product_revision'] == bootstrap['tooling_revision'] == source
assert release['binary_sha256'] == '7c649882cf08ad4cdbe43c619c8d8f229f97aa96e5e27bacf2f5054a1b95e0ef'
archive = root / 'runtime.tar'
with tarfile.open(archive) as tar:
    manifest = json.load(tar.extractfile('manifest.json'))
    assert len(manifest) == 1
    config = tar.extractfile(manifest[0]['Config']).read()
    image = json.loads(config)
    assert image['architecture'] == 'arm64' and image['os'] == 'linux'
    (root / 'image-id').write_text('sha256:' + hashlib.sha256(config).hexdigest())
(root / 'archive-sha256').write_text(hashlib.file_digest(archive.open('rb'), 'sha256').hexdigest() + '\n')
PY
docker load -i "$root/artifact/runtime.tar" > "$root/artifact/docker-load.log"
image=$(cat "$root/artifact/image-id")
container="biomcp-cache-first-open-$$"
cleanup() { docker rm -f "$container" >/dev/null 2>&1 || true; }
trap cleanup EXIT HUP INT TERM
docker run --rm ${JOB_LABEL_FILE:+--label-file "$JOB_LABEL_FILE"} --detach \
    --name "$container" --platform linux/arm64 --memory 1g --cpus 1 \
    --user "$(id -u):$(id -g)" --network none \
    -v "$root:$root" --entrypoint /bin/sleep "$image" 1200
python3 scripts/cache_installed_first_open.py /usr/local/bin/biomcp \
    7c649882cf08ad4cdbe43c619c8d8f229f97aa96e5e27bacf2f5054a1b95e0ef \
    "$root/results" --container "$container" \
    --artifact "yellow:$source_base/dist/biomcp-container-arm64.tar"

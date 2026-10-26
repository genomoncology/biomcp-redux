#!/bin/sh
# Replay accepted source replies through transport-owned recordings; no provider requests.
set -eu
mkdir -p "$HOME/job-evidence/biomcp-redux-0241"
proof_root=${1:-}
if [ -z "$proof_root" ]; then
    proof_root=$(mktemp -d "$HOME/job-evidence/biomcp-redux-0241/native.XXXXXX")
    mkdir "$proof_root/archives"
    scp yellow:/home/ian/workspace/data/biomcp-redux/0190-protein-investigation/j14202-native-sources.tar.gz yellow:/home/ian/workspace/data/biomcp-redux/0190-protein-investigation/j14211-j14221-identity-followup.tar.gz yellow:/home/ian/workspace/data/biomcp-redux/0229-protein-admission/j14362-native-witnesses.tar.gz "$proof_root/archives/"
fi
export proof_root
printf 'Retained proof directory: %s\n' "$proof_root"
python3 - <<'PY'
import hashlib, os
from pathlib import Path
root = Path(os.environ['proof_root'])
for name, expected in {
    'j14202-native-sources.tar.gz':'25d2cd2bb2a1236e6aee9fe84ffc7879723df92bd031396da3e41b0c6ff989c8',
    'j14211-j14221-identity-followup.tar.gz':'2558f595fd554a648f6477fb1fcd2e6e3645b425f64d8df0ffd6375575dbdafe',
    'j14362-native-witnesses.tar.gz':'2808c280a38e69e533562269c2376590fff28e4661fbb4a61dc5705bf46a3047',
}.items():
    if hashlib.sha256((root/'archives'/name).read_bytes()).hexdigest() != expected:
        raise ValueError('accepted native archive changed: '+name)
PY
if [ ! -d "$proof_root/protein-domain-native" ]; then
    python3 scripts/protein_domains_native.py "$proof_root/archives" "$proof_root/protein-domain-native"
fi
if [ -f "$proof_root/proof.log" ]; then
    prior_log=$(mktemp "$proof_root/prior-proof.XXXXXX")
    cp "$proof_root/proof.log" "$prior_log"
fi
git rev-parse HEAD >> "$proof_root/revisions.txt"
BIOMCP_TEST_FIXTURES="$proof_root" cargo test --locked -p biomcp --test cases -- protein-domains-complexes/retained-native-facts-and-costs --nocapture > "$proof_root/proof.log" 2>&1 || { cat "$proof_root/proof.log"; exit 1; }
cat "$proof_root/proof.log"
python3 - <<'PY'
import hashlib, os, shutil, sys, tarfile
from pathlib import Path
sys.path.insert(0,'scripts')
import source_capture
root=Path(os.environ['proof_root'])
evidence=root/'evidence'
evidence.mkdir(exist_ok=True)
for folder,prefix in [('cases','case'),('outputs','result')]:
    for path in (root/'protein-domain-native'/folder).iterdir():
        shutil.copyfile(path,evidence/(prefix+'-'+path.name))
for source in (root/'protein-domain-native/replay').iterdir():
    for path in source.iterdir(): shutil.copyfile(path,evidence/('replay-'+source.name+'-'+path.name))
for path in [root/'proof.log',*root.glob('prior-proof.*'),root/'revisions.txt']: shutil.copyfile(path,evidence/path.name)
source_capture.manifest(evidence)
archive=root/'native-proof.tar.gz'
with tarfile.open(archive,'w:gz') as packed:
    for path in sorted(evidence.iterdir()): packed.add(path,arcname=path.name,recursive=False)
source_capture.verify_archive(archive)
(root/'archive-sha.txt').write_text(hashlib.sha256(archive.read_bytes()).hexdigest())
PY
archive_name="$(basename "$proof_root")-$(git rev-parse --short HEAD).tar.gz"
ssh yellow 'mkdir -p /home/ian/workspace/data/biomcp-redux/0241-protein-domains-complexes'
scp "$proof_root/native-proof.tar.gz" "yellow:/home/ian/workspace/data/biomcp-redux/0241-protein-domains-complexes/$archive_name"
ssh yellow python3 - "$archive_name" "$(cat "$proof_root/archive-sha.txt")" <<'PY'
import hashlib,json,sys,tarfile
from pathlib import Path
path=Path('/home/ian/workspace/data/biomcp-redux/0241-protein-domains-complexes')/sys.argv[1]
assert hashlib.sha256(path.read_bytes()).hexdigest()==sys.argv[2]
with tarfile.open(path) as archive:
    names=archive.getnames(); manifest=json.load(archive.extractfile('manifest.json'))
    assert len(names)==len(set(names)) and set(names)==set(manifest)|{'manifest.json'}
    for name,expected in manifest.items(): assert hashlib.sha256(archive.extractfile(name).read()).hexdigest()==expected
print('Verified native archive',path.name,'members',len(names),'sha256',sys.argv[2])
PY

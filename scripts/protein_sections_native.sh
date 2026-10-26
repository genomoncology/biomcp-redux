#!/bin/sh
# Reuse retained source replies; make no provider request.
set -eu
mkdir -p "$HOME/workspace/job-evidence"
proof_root=$(mktemp -d "$HOME/workspace/job-evidence/biomcp-redux-0239-native.XXXXXX")
previous_proof=${1:-}
export proof_root previous_proof
printf 'Retained proof directory: %s\n' "$proof_root"
mkdir "$proof_root/archives"
scp yellow:/home/ian/workspace/data/biomcp-redux/0190-protein-investigation/j14202-native-sources.tar.gz yellow:/home/ian/workspace/data/biomcp-redux/0190-protein-investigation/j14211-j14221-identity-followup.tar.gz yellow:/home/ian/workspace/data/biomcp-redux/0229-protein-admission/j14362-native-witnesses.tar.gz "$proof_root/archives/"
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
        raise ValueError('accepted native archive hash changed: '+name)
PY
python3 scripts/protein_sections_native.py "$proof_root/archives" "$proof_root/protein-section-native"
git rev-parse HEAD > "$proof_root/revision.txt"
python3 - <<'PY_RESUME'
import os, shutil
from pathlib import Path
previous = os.environ['previous_proof']
if previous:
    old = Path(previous).resolve()
    parent = Path.home()/'workspace/job-evidence'
    if old.parent != parent or not old.name.startswith('biomcp-redux-0239-native.'):
        raise ValueError('continuation must name this lane own retained evidence')
    root = Path(os.environ['proof_root'])
    reused = root/'protein-section-native/reused-cases'
    reused.mkdir()
    for result in (old/'protein-section-native/outputs').glob('*.json'):
        case = root/'protein-section-native/cases'/(result.stem+'.yaml')
        if case.is_file(): shutil.move(case, reused/case.name)
    for path in [*old.glob('*proof*.log'), *old.glob('*revision*.txt')]:
        shutil.copyfile(path, root/('prior-'+path.name))
    print('Reuse', len(list(reused.iterdir())), 'completed native cases from', old.name)
PY_RESUME
BIOMCP_TEST_FIXTURES="$proof_root" cargo test --locked -p biomcp --test cases -- protein-sections/retained-native-facts-and-costs --nocapture > "$proof_root/proof.log" 2>&1 || { cat "$proof_root/proof.log"; exit 1; }
cat "$proof_root/proof.log"
python3 - <<'PY'
import hashlib, os, shutil, sys, tarfile
from pathlib import Path
sys.path.insert(0, 'scripts')
import source_capture
root = Path(os.environ['proof_root'])
evidence = root/'evidence'
evidence.mkdir()
previous = os.environ['previous_proof']
if previous:
    old = Path(previous)
    for path in (old/'protein-section-native/outputs').glob('*.json'):
        shutil.copyfile(path, root/'protein-section-native/outputs'/path.name)
for path in [*root.glob('*proof*.log'), *root.glob('*revision*.txt')]:
    shutil.copyfile(path, evidence/path.name)
for kind, folder in [('case', 'cases'), ('reused', 'reused-cases'), ('walk', 'walks'), ('result', 'outputs'), ('replay', 'replay/uniprot')]:
    directory = root/'protein-section-native'/folder
    if not directory.is_dir(): continue
    for path in directory.iterdir():
        shutil.copyfile(path, evidence/(kind+'-'+path.name))
source_capture.manifest(evidence)
archive = root/'native-proof.tar.gz'
with tarfile.open(archive, 'w:gz') as packed:
    for path in sorted(evidence.iterdir()):
        packed.add(path, arcname=path.name, recursive=False)
source_capture.verify_archive(archive)
(root/'archive-sha.txt').write_text(hashlib.sha256(archive.read_bytes()).hexdigest())
PY
archive_name="$(basename "$proof_root").tar.gz"
ssh yellow 'mkdir -p /home/ian/workspace/data/biomcp-redux/0239-uniprot-sections'
scp "$proof_root/native-proof.tar.gz" "yellow:/home/ian/workspace/data/biomcp-redux/0239-uniprot-sections/$archive_name"
ssh yellow python3 - "$archive_name" "$(cat "$proof_root/archive-sha.txt")" <<'PY'
import hashlib, json, sys, tarfile
from pathlib import Path
path = Path('/home/ian/workspace/data/biomcp-redux/0239-uniprot-sections')/sys.argv[1]
assert hashlib.sha256(path.read_bytes()).hexdigest() == sys.argv[2]
with tarfile.open(path) as archive:
    names = archive.getnames()
    manifest = json.load(archive.extractfile('manifest.json'))
    assert len(names) == len(set(names)) and set(names) == set(manifest)|{'manifest.json'}
    for name, expected in manifest.items():
        assert hashlib.sha256(archive.extractfile(name).read()).hexdigest() == expected
print('Verified native archive',path.name,'members',len(names),'sha256',sys.argv[2])
PY
rm -rf "$proof_root"

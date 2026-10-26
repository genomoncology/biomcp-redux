"""Capture only two publisher-documented GPML files and preserve them on Yellow."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tarfile
import time
import xml.etree.ElementTree as ET

from source_capture import verify_archive


URLS = [
    ('WP382', 'https://www.wikipathways.org/wikipathways-assets/pathways/WP382/WP382.gpml'),
    ('WP100', 'https://www.wikipathways.org/wikipathways-assets/pathways/WP100/WP100.gpml'),
]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parent = Path.home() / 'job-evidence' / 'biomcp-redux-0224'
    parent.mkdir(parents=True, exist_ok=True)
    root = parent / ('native-' + str(time.time_ns()))
    root.mkdir()
    receipts = []
    failed = False
    for index, (identity, url) in enumerate(URLS):
        if index:
            time.sleep(1)
        plan = {'deadline_seconds': 8, 'max_bytes': 8 * 1024 * 1024,
                'spacing_seconds': 1, 'requests': [
                    {'id': identity, 'source': 'wikipathways', 'url': url,
                     'response_format': 'bytes'}]}
        plan_file = root / (identity + '-plan.json')
        plan_file.write_text(json.dumps(plan))
        out = root / identity
        result = subprocess.run(['python3', 'scripts/source_capture.py',
                                 '--plan', str(plan_file), '--out', str(out)])
        archive = Path(str(out) + '.tar.gz')
        if archive.exists():
            verify_archive(archive)
        receipt = {'id': identity, 'url': url, 'capture_exit': result.returncode}
        body = out / '0000.body'
        try:
            raw = body.read_bytes()
            if b'<!DOCTYPE' in raw or b'<!ENTITY' in raw:
                raise ValueError('external entity declarations are forbidden')
            document = ET.fromstring(raw)
            nodes = document.findall('{http://pathvisio.org/GPML/2013a}DataNode')
            kinds = {}
            for node in nodes:
                kind = node.get('Type')
                kinds[kind] = kinds.get(kind, 0) + 1
            receipt.update(bytes=len(raw), sha256=digest(body),
                           attributes=document.attrib, node_kinds=kinds)
            if (result.returncode or document.get('Organism') != 'Homo sapiens'
                    or not document.get('Version', '').startswith(identity + '_r')):
                raise ValueError('capture or GPML identity failed')
        except (OSError, ValueError, ET.ParseError) as error:
            receipt['failure'] = str(error)
            failed = True
        receipts.append(receipt)
        if failed:
            break
    (root / 'summary.json').write_text(json.dumps(receipts, indent=2))
    files = [p for p in root.iterdir() if p.is_file()]
    manifest = {p.name: digest(p) for p in files}
    (root / 'manifest.json').write_text(json.dumps(manifest, indent=2))
    archive = Path(str(root) + '.tar.gz')
    with tarfile.open(archive, 'w:gz') as packed:
        for path in sorted(root.iterdir()):
            if path.is_file():
                packed.add(path, arcname=path.name)
    verify_archive(archive)
    destination = '/home/ian/workspace/data/biomcp-redux/0224-wikipathways-genes'
    subprocess.run(['ssh', 'yellow', 'mkdir', '-p', destination], check=True)
    subprocess.run(['scp', str(archive), 'yellow:' + destination + '/'], check=True)
    verifier = '''import hashlib,json,sys,tarfile
p=sys.argv[1]
assert hashlib.sha256(open(p,'rb').read()).hexdigest()==sys.argv[2]
with tarfile.open(p) as t:
 names=t.getnames(); manifest=json.load(t.extractfile('manifest.json'))
 assert len(names)==len(set(names)) and set(names)==set(manifest)|{'manifest.json'}
 assert all(m.isfile() and '/' not in m.name for m in t.getmembers())
 for name,value in manifest.items():
  assert hashlib.sha256(t.extractfile(name).read()).hexdigest()==value
print('Verified exact members and hashes on Yellow')
'''
    subprocess.run(['ssh', 'yellow', 'python3', '-', destination + '/' + archive.name,
                    digest(archive)], input=verifier, text=True, check=True)
    print(json.dumps({'archive': destination + '/' + archive.name,
                      'sha256': digest(archive), 'observations': receipts}))
    if failed:
        raise SystemExit(2)


if __name__ == '__main__':
    os.environ['COPYFILE_DISABLE'] = '1'
    main()

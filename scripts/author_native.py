"""Prepare unchanged retained native author replies outside Git for the owning cases."""
import hashlib
import json
from pathlib import Path
import sys
import tarfile
import source_capture

archives, output = map(Path, sys.argv[1:3])
kind = sys.argv[3] if len(sys.argv) > 3 else 'all'
if kind not in ['all', 'orcid', 'author-papers']:
    raise ValueError('unknown native case kind')
source_capture.outside_git(output.resolve())
output.mkdir()
replay = output / 'replay' / 'semantic-scholar'
replay.mkdir(parents=True)
cases = output / 'cases'
cases.mkdir()
accepted = {
    '2290-author-captures-redacted.tar.gz': '0a873520c1dacb33c93a22f00868ebae58401f21ad0b768026ac8c52b75c3964',
    '2290-author-dependents.tar.gz': 'd8521464f4f2525a13450efe352abcaa97170a8d3232fde02e9b299202f86eb3',
    '0266-author-card-xowlaemk.tar.gz': 'c077d787ebf31f8308c90a22a9c1085d8fff3ea8c50626afae9d5984f73a4d2a',
}
records = {}
for name, expected in accepted.items():
    packed = archives / name
    if hashlib.sha256(packed.read_bytes()).hexdigest() != expected:
        raise ValueError('accepted native archive differs: ' + name)
    with tarfile.open(packed) as archive:
        manifest = json.load(archive.extractfile('manifest.json'))
        if set(archive.getnames()) != set(manifest) | {'manifest.json'}:
            raise ValueError('archive inventory differs')
        for member, digest in manifest.items():
            if hashlib.sha256(archive.extractfile(member).read()).hexdigest() != digest:
                raise ValueError('archive member differs: ' + member)
        ledger = json.load(archive.extractfile('ledger.json'))
        for row in ledger:
            if row['source'] != 'orcid' and not row['id'].startswith(('search-', 'card-', 'absent-author', 'papers-')):
                continue
            raw = archive.extractfile(f"{row['index']:04d}.body").read()
            if hashlib.sha256(raw).hexdigest() != row['body_sha256']:
                raise ValueError('native body differs')
            # Text encoding preserves every native JSON body byte, including whitespace.
            record = {'method': 'GET', 'source': row['source'], 'url': row['url'],
                      'exchanges': [{'status': row['status'], 'content_type': 'application/json',
                                     'delay_ms': 0, 'retry_after_ms': None,
                                     'body': {'kind': 'text', 'value': raw.decode()}}]}
            key = hashlib.sha256(b'GET\0' + row['url'].encode()).hexdigest()
            destination = replay.parent / row['source']
            destination.mkdir(exist_ok=True)
            (destination / (key + '.json')).write_text(json.dumps(record) + '\n')
            records[row['id']] = (row, json.loads(raw))

templates = Path(__file__).resolve().parent.parent / 'tests/native/author'
for template in [*templates.glob('*.yaml'), *templates.with_name('author-papers').glob('*.yaml')]:
    if kind != 'all' and template.parent.name != kind:
        continue
    case = json.loads(template.read_text())
    case['replay'] = str(replay.parent)
    (cases / template.name).write_text(json.dumps(case, indent=2) + '\n')

# One bounded formatter/discovery measurement uses the existing canonical help owners.
for name, source, indices in [
    ('author-search-help', 'tests/cases/author/help.yaml', [0]),
    ('author-get-help', 'tests/cases/author/help.yaml', [1]),
    ('author-list-help', 'tests/cases/author/help.yaml', [2]),
    ('whole-help', 'tests/cases/gene/help.yaml', [0]),
]:
    if kind != 'all':
        continue
    case = json.loads((Path(__file__).resolve().parent.parent / source).read_text())
    case['steps'] = [case['steps'][index] for index in indices]
    case.pop('replay', None)
    (cases / (name + '.yaml')).write_text(json.dumps(case, indent=2) + '\n')

# ORCID assertions are licensed source records. Build the native owning cases here,
# so names and native assertion values remain in protected staging outside Git.
import orcid_native
if kind in ['all', 'orcid']:
    orcid_native.prepare(records, cases, replay.parent)

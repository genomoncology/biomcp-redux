"""Prepare unchanged retained native author replies outside Git for the owning cases."""
import hashlib
import json
from pathlib import Path
import sys
import tarfile

archives, output = map(Path, sys.argv[1:])
output.mkdir()
replay = output / 'replay' / 'semantic-scholar'
replay.mkdir(parents=True)
cases = output / 'cases'
cases.mkdir()
accepted = {
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
            if not row['id'].startswith(('search-', 'card-', 'absent-author')):
                continue
            raw = archive.extractfile(f"{row['index']:04d}.body").read()
            if hashlib.sha256(raw).hexdigest() != row['body_sha256']:
                raise ValueError('native body differs')
            # Text encoding preserves every native JSON body byte, including whitespace.
            record = {'method': 'GET', 'source': 'semantic-scholar', 'url': row['url'],
                      'exchanges': [{'status': row['status'], 'content_type': 'application/json',
                                     'delay_ms': 0, 'retry_after_ms': None,
                                     'body': {'kind': 'text', 'value': raw.decode()}}]}
            key = hashlib.sha256(b'GET\0' + row['url'].encode()).hexdigest()
            (replay / (key + '.json')).write_text(json.dumps(record) + '\n')
            records[row['id']] = (row, json.loads(raw))

budgets_path = Path(__file__).resolve().parent.parent / 'tests/author-command-budgets.json'
budgets = json.loads(budgets_path.read_text()) if budgets_path.exists() else {}

def case(name, args, expect, exit_code=0):
    step = {'run': args, 'mcp': args, 'exit': exit_code, 'is_error': exit_code >= 2,
            'expect': expect, 'full_output_bytes': True}
    if name in budgets:
        ceiling = (budgets[name] * 115 + 99) // 100
        step.update(max_bytes=ceiling, max_tokens=(ceiling + 3) // 4)
    (cases / (name + '.yaml')).write_text(json.dumps({
        'feature': 'author-search', 'harvest': 'sdlc/planning/slices/author.md A01 A03 A05 A07 A09 A11 A16 A21',
        'story': 'Retained exact provider records preserve native identity and counts without linking people',
        'replay': str(replay.parent), 'steps': [step]}, indent=2) + '\n')

for offset in [0, 5]:
    _, body = records['search-' + str(offset)]
    args = ['search', 'author', 'Louis Williams'] + (['--offset', str(offset)] if offset else [])
    case('search-' + str(offset), args, {
        '/status': 'complete', '/page/offset': offset, '/page/returned': 5,
        '/result/items/*/id': ['semantic-scholar:' + r['authorId'] for r in body['data']],
        '/result/matching_records': body['total'], '/next/5/kind': 'page'})
for pmid in ['17611497', '33123537', '34096690']:
    _, body = records['card-' + pmid]
    qualified = 'semantic-scholar:' + body['authorId']
    case('card-' + pmid, ['get', 'author', qualified], {
        '/status': 'complete', '/result/items/0/input': qualified,
        '/result/items/0/card/header/id': qualified, '/result/items/0/card/header/name': body['name'],
        '/result/items/0/card/header/counts/scope': 'provider_record',
        '/result/items/0/card/header/counts/paper_count': body['paperCount'],
        '/result/items/0/card/header/counts/citation_count': body['citationCount'],
        '/result/items/0/card/url': body['url'], '/next/0/kind': 'pivot'})
case('absent', ['get', 'author', 'semantic-scholar:999999999999999999'], {
    '/status': 'absent', '/result/items/0/status': 'absent', '/sources/0/state': 'ok'}, 1)

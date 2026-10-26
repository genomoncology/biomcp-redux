"""Run the approved two-phase author capture once and retain every source reply."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import time

import source_capture as capture


def pack(folder, archive):
    capture.manifest(folder)
    with tarfile.open(archive, 'w:gz') as target:
        for path in sorted(folder.iterdir()):
            target.add(path, arcname=path.name, recursive=False)
    capture.verify_archive(archive)


def initialize():
    root = Path.home() / 'workspace/experiments'
    root.mkdir(exist_ok=True)
    numbers = [int(p.name.split('-')[0]) for p in root.iterdir()
               if p.name.split('-')[0].isdigit()]
    folder = root / ('%03d-author-captures' % (max(numbers + [0]) + 1))
    folder.mkdir()
    (folder / 'OWNER').write_text('biomcp-redux/0264\n')
    for name in ('author_capture_plan.json', 'capture_author_witnesses.py',
                 'source_capture.py', 'capture_outcome.py'):
        shutil.copyfile(Path('scripts') / name, folder / name)
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True)
    (folder / 'revision.txt').write_text(revision)
    return folder


def materialize(recipe, ids):
    rows = []
    for original in recipe:
        row = dict(original)
        row['url'] = row['url'].format(**ids)
        key = row.get('key')
        if key and not os.environ.get(key['env']):
            del row['key']
        rows.append(row)
    return rows


def request(row, recipe, folder, ledger):
    plan = {key: recipe[key] for key in
            ('deadline_seconds', 'max_bytes', 'spacing_seconds')}
    plan['requests'] = [row]
    path = folder / 'active-plan.json'
    capture.write_json(path, plan)
    plan, secrets = capture.load_plan(path)
    if ledger:
        time.sleep(recipe['spacing_seconds'] if row['source'] == 'semantic-scholar' else 0.5)
    receipt = capture.capture_one(len(ledger), row, plan, secrets, folder)
    body = json.loads((folder / ('%04d.body' % len(ledger))).read_bytes()) if (
        receipt['outcome'] == 'success') else {}
    if not isinstance(body, dict):
        receipt['outcome'], body = 'malformed', {}
    receipt['capability'] = 'proven' if receipt['outcome'] == 'success' else 'failed'
    receipt['returned_author_ids'] = author_ids(body)
    receipt['native_total'] = body.get('total')
    receipt['returned_rows'] = len(body.get('data', [])) if 'data' in body else None
    receipt['native_offset'] = body.get('offset')
    receipt['native_next'] = body.get('next')
    receipt['returned_orcid_path'] = body.get('path')
    receipt['orcid_identifier'] = body.get('orcid-identifier')
    receipt['work_group_count'] = len(body.get('group', [])) if 'group' in body else None
    ledger.append(receipt)
    capture.write_json(folder / 'ledger.json', ledger)
    return receipt, body


def author_ids(body):
    found = []
    def walk(value):
        if isinstance(value, dict):
            if 'authorId' in value:
                found.append(value['authorId'])
            for child in value.values():
                walk(child)
        elif isinstance(value, list):
            for child in value:
                walk(child)
    walk(body)
    return found


def bylines(recipe, folder, ledger):
    ids = dict(recipe.get('reused_first_author_ids', {}))
    for row in materialize(recipe['phase1'], {}):
        receipt, body = request(row, recipe, folder, ledger)
        pmid = row['id'].removeprefix('byline-')
        authors = body.get('authors') or []
        first = authors[0].get('authorId') if authors else None
        if (receipt['outcome'] != 'success' or not isinstance(first, str)
                or not first.isdigit() or body.get('externalIds', {}).get('PubMed') != pmid):
            return None
        ids['first_' + pmid] = first
    return ids


def second_phase(rows, recipe, folder, ledger):
    stopped = set()
    previous = {}
    for row in rows:
        source = row['source']
        if source in stopped:
            continue
        predecessor = {'search-5': 'search-0',
                       'papers-32955176-5': 'papers-32955176-0'}.get(row['id'])
        if predecessor and previous.get(predecessor, {}).get('next') != 5:
            stopped.add(source)
            continue
        receipt, body = request(row, recipe, folder, ledger)
        previous[row['id']] = body
        if row['id'].startswith(('search-', 'papers-')) and receipt['outcome'] == 'success':
            offset = 5 if row['id'] in ('search-5', 'papers-32955176-5') else 0
            data = body.get('data')
            if not isinstance(data, list) or len(data) > 5 or body.get('offset') != offset:
                receipt['capability'] = 'failed: native page does not match request'
                stopped.add(source)
        if row['id'] == 'papers-34096690-0-details' and receipt['outcome'] == 'success':
            compact = previous.get('papers-34096690-0', {}).get('data', [])
            if [p.get('paperId') for p in body.get('data', [])] != [p.get('paperId') for p in compact]:
                receipt['capability'] = 'failed: details changed the paper page'
                stopped.add(source)
        if row['id'].startswith('card-') and body.get('authorId') != row['url'].split('/author/')[1].split('?')[0]:
            receipt['capability'] = 'failed: returned author identity differs'
            stopped.add(source)
        if row['id'].startswith('absent-'):
            receipt['capability'] = ('empty' if receipt['status'] == 404 else
                                     'proven-positive; absence unproved' if receipt['outcome'] == 'success'
                                     else 'failed; absence unproved')
        elif receipt['outcome'] != 'success' or body.get('data') == []:
            receipt['capability'] = 'empty' if body.get('data') == [] else 'failed'
            stopped.add(source)
    capture.write_json(folder / 'ledger.json', ledger)


def retain(folder):
    archive = Path(str(folder) + '.tar.gz')
    pack(folder, archive)
    destination = '/home/ian/workspace/data/biomcp-redux/0264-author-captures'
    subprocess.run(['ssh', 'yellow', 'mkdir', '-p', destination], check=True)
    subprocess.run(['scp', str(archive), 'yellow:' + destination + '/'], check=True)
    remote = destination + '/' + archive.name
    checksum = capture.file_digest(archive)
    verify = '''import hashlib,io,json,sys,tarfile
p,expected=sys.argv[1:]
assert hashlib.sha256(open(p,'rb').read()).hexdigest()==expected
with tarfile.open(p) as t:
 m=json.load(t.extractfile('manifest.json')); names=t.getnames()
 assert len(names)==len(set(names)) and set(names)==set(m)|{'manifest.json'}
 for n,h in m.items():
  assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
 raw=t.extractfile('phase2-inventory.tar.gz').read()
with tarfile.open(fileobj=io.BytesIO(raw)) as t:
 m=json.load(t.extractfile('manifest.json')); names=t.getnames()
 assert len(names)==len(set(names)) and set(names)==set(m)|{'manifest.json'}
 for n,h in m.items():
  assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
print('Every archive and inventory member verified independently')
'''
    subprocess.run(['ssh', 'yellow', 'python3', '-', remote, checksum],
                   input=verify.encode(), check=True)
    print(json.dumps({'archive': remote, 'sha256': checksum,
                      'removed_staging': [str(folder), str(archive)]}))
    shutil.rmtree(folder)
    archive.unlink()


def main():
    capture.require_environment()
    recipe = json.loads(Path('scripts/author_capture_plan.json').read_text())
    assert len(recipe['phase1'] + recipe['phase2']) == recipe['max_requests'] == 20
    folder = initialize()
    ledger = []
    capture.write_json(folder / 'credential-presence.json', {
        key: bool(os.environ.get(key)) for key in ('S2_API_KEY', 'ORCID_ACCESS_TOKEN')})
    ids = bylines(recipe, folder, ledger)
    capture.write_json(folder / 'first-author-ids.json', ids)
    rows = materialize(recipe['phase2'], ids) if ids else materialize(
        [r for r in recipe['phase2'] if r['source'] == 'orcid'], {})
    inventory = folder / 'inventory'
    inventory.mkdir()
    capture.write_json(inventory / 'phase2-exact.json', rows)
    pack(inventory, folder / 'phase2-inventory.tar.gz')
    shutil.rmtree(inventory)
    second_phase(rows, recipe, folder, ledger)
    attempted = {r['id'] for r in ledger}
    capture.write_json(folder / 'deferred.json', [
        {'id': r['id'], 'url': r['url'], 'reason': 'required earlier witness failed, empty or lacked native continuation'}
        for r in recipe['phase1'] + rows if r['id'] not in attempted])
    print(json.dumps({'attempted': len(ledger), 'rows': [
        {'id': r['id'], 'status': r['status'], 'capability': r['capability']}
        for r in ledger]}))
    retain(folder)


if __name__ == '__main__':
    main()

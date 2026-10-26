"""Run the one approved remaining-author follow-up after recipe review."""
from datetime import datetime, timezone
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile

import capture_author_witnesses as author
import source_capture as capture


def now():
    return datetime.now(timezone.utc).isoformat()


def reused_evidence(recipe):
    previous = recipe['reused_archive']
    raw = subprocess.check_output(['ssh', 'yellow', 'cat', previous['path']])
    if hashlib.sha256(raw).hexdigest() != previous['sha256']:
        raise ValueError('protected source archive hash differs')
    with tarfile.open(fileobj=io.BytesIO(raw)) as packed:
        manifest = json.load(packed.extractfile('manifest.json'))
        names = packed.getnames()
        if len(names) != len(set(names)) or set(names) != set(manifest) | {'manifest.json'}:
            raise ValueError('protected source archive members differ')
        for name, expected in manifest.items():
            if hashlib.sha256(packed.extractfile(name).read()).hexdigest() != expected:
                raise ValueError('protected source archive member differs')
        body_raw = packed.extractfile('0000.body').read()
        body = json.loads(body_raw)
        first = body['authors'][0]['authorId']
        if (body['externalIds']['PubMed'] != '17611497'
                or first != recipe['reused_first_author_ids']['first_17611497']):
            raise ValueError('retained first-row identity differs')
    return dict(previous, member='0000.body', body_sha256=capture.digest(body_raw),
                first_author_id=first, all_orcid_requests_reused=True)


def prelaunch(folder, args):
    raw = args.prelaunch_snapshot.read_bytes()
    if capture.digest(raw) != args.prelaunch_sha256:
        raise ValueError('prelaunch snapshot hash differs')
    evidence = folder / 'prelaunch'
    evidence.mkdir()
    (evidence / 'owned-job-snapshot.json').write_bytes(raw)
    author.pack(evidence, folder / 'prelaunch-snapshot.tar.gz')
    shutil.rmtree(evidence)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--prelaunch-snapshot', type=Path, required=True)
    parser.add_argument('--prelaunch-sha256', required=True)
    args = parser.parse_args()
    capture.require_environment()
    if not os.environ.get('S2_API_KEY'):
        raise ValueError('follow-up requires the named Semantic Scholar key')
    recipe = json.loads(Path('scripts/author_followup_plan.json').read_text())
    planned = recipe['phase1'] + recipe['phase2']
    if len(planned) != recipe['max_requests'] or len(planned) != 14:
        raise ValueError('follow-up must contain exactly fourteen planned rows')
    if any(r['source'] != 'semantic-scholar' or r['id'] == 'byline-17611497' for r in planned):
        raise ValueError('follow-up cannot repeat ORCID or the retained byline')
    reused = reused_evidence(recipe)
    folder = author.initialize()
    for name in ('author_followup_plan.json', 'capture_author_followup.py'):
        shutil.copyfile(Path('scripts') / name, folder / name)
    capture.write_json(folder / 'reused-evidence.json', reused)
    capture.write_json(folder / 'credential-presence.json', {
        'S2_API_KEY': True, 'ORCID_ACCESS_TOKEN': bool(os.environ.get('ORCID_ACCESS_TOKEN'))})
    window = dict(recipe['concurrency'], capture_started_at_utc=now())
    capture.write_json(folder / 'concurrency-window.json', window)
    prelaunch(folder, args)
    window['prelaunch_snapshot_sha256'] = args.prelaunch_sha256
    capture.write_json(folder / 'concurrency-window.json', window)
    ledger = []
    ids = author.bylines(recipe, folder, ledger)
    capture.write_json(folder / 'first-author-ids.json', ids)
    rows = author.materialize(recipe['phase2'], ids) if ids else []
    inventory = folder / 'inventory'
    inventory.mkdir()
    capture.write_json(inventory / 'phase2-exact.json', rows)
    author.pack(inventory, folder / 'phase2-inventory.tar.gz')
    shutil.rmtree(inventory)
    author.second_phase(rows, recipe, folder, ledger)
    window['capture_finished_at_utc'] = now()
    window['attempted_requests'] = len(ledger)
    capture.write_json(folder / 'concurrency-window.json', window)
    attempted = {r['id'] for r in ledger}
    capture.write_json(folder / 'deferred.json', [
        {'id': r['id'], 'url_recipe': r['url'],
         'reason': 'required earlier witness failed, empty or lacked native continuation'}
        for r in planned if r['id'] not in attempted])
    print(json.dumps({'attempted': len(ledger), 'rows': [
        {k: r[k] for k in ('id', 'status', 'capability', 'credential_header',
                           'credential_header_attached')} for r in ledger]}))
    author.retain(folder)


if __name__ == '__main__':
    main()

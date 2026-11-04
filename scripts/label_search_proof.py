"""Exercise source label pages against one installed CLI and retain source bindings."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import urllib.error
import urllib.parse
import urllib.request

PROOF = Path('target/label-search-proof')


def focused():
    artifacts = [json.loads(line) for line in (PROOF / 'build.jsonl').read_text().splitlines()]
    executable = next(Path(row['executable']) for row in artifacts
                      if row.get('reason') == 'compiler-artifact' and row.get('executable')
                      and row['target']['name'] == 'biomcp' and row['target']['kind'] == ['bin'])
    runner = next(Path(row['executable']) for row in artifacts
                  if row.get('reason') == 'compiler-artifact' and row.get('executable')
                  and row['target']['name'] == 'cases')
    installed = PROOF / 'installed/biomcp'
    installed.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(executable, installed)
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    (PROOF / 'source-bindings.json').write_text(json.dumps({
        'source_commit': revision, 'binary_sha256': hashlib.sha256(installed.read_bytes()).hexdigest(),
        'executable': str(installed.resolve()), 'runner': str(runner)}, indent=2) + '\n')
    command = [sys.executable, 'scripts/trials.py', '--runner', str(runner),
               '--executable', str(installed), '--installed']
    listed = subprocess.check_output([*command, '--list'], text=True)
    (PROOF / 'actual-list.txt').write_text(listed)
    names = [line.removesuffix(': test') for line in listed.splitlines()
             if line.endswith(': test') and ('drug/label-' in line or 'drug/openfda-label' in line
                                              or 'drug/help' in line)]
    if not names:
        raise ValueError('no label trials discovered')
    subprocess.run([*command, *names], check=True)


def live():
    executable = (PROOF / 'installed/biomcp').resolve()
    for query in ['imatinib', 'chronic myeloid leukemia']:
        for limit in ([3, 10] if query == 'imatinib' else [10]):
            seen = set()
            offset = 0
            total = None
            with tempfile.TemporaryDirectory(prefix='label-search-live-') as isolated:
                env = dict(os.environ, HOME=isolated, XDG_CONFIG_HOME=isolated,
                           XDG_CACHE_HOME=isolated, XDG_DATA_HOME=isolated)
                while True:
                    command = [str(executable), 'search', 'drug', query, '--source', 'openfda-label',
                               '--limit', str(limit), '--offset', str(offset)]
                    result = subprocess.run(command, env=env, capture_output=True, check=True)
                    stem = f'{query.replace(" ", "-")}-{limit}-{offset}'
                    (PROOF / f'{stem}.json').write_bytes(result.stdout)
                    reply = json.loads(result.stdout)
                    page = reply['page']
                    items = reply['result'].get('items', [])
                    current = page['total']
                    if total is not None and total != current:
                        raise ValueError('native total changed across pages')
                    total = current
                    if len(items) != min(limit, total - offset) or any(not row.get('name') for row in items):
                        raise ValueError('page is incomplete or unnamed')
                    ids = [row['id'] for row in items]
                    if len(set(ids)) != len(ids) or seen.intersection(ids):
                        raise ValueError('source page repeats an SPL set')
                    seen.update(ids)
                    if offset == 0:
                        selectors = [f'openfda.brand_name{suffix}:"{query}" OR openfda.generic_name{suffix}:"{query}"'
                                     for suffix in ['.exact', '']]
                        selectors.append('"' + query + '"')
                        for index, selector in enumerate(selectors):
                            url = 'https://api.fda.gov/drug/label.json?' + urllib.parse.urlencode({'search': selector, 'limit': 1})
                            try:
                                with urllib.request.urlopen(url) as response:
                                    native = response.read()
                            except urllib.error.HTTPError as error:
                                if error.code != 404:
                                    raise
                                native = error.read()
                            (PROOF / f'{stem}-source-{index}.json').write_bytes(native)
                            native_total = json.loads(native).get('meta', {}).get('results', {}).get('total', 0)
                            if native_total:
                                break
                        (PROOF / f'{stem}-source-selector.json').write_text(json.dumps({'url': url, 'selector': selector, 'total': native_total}))
                        if native_total != total:
                            raise ValueError('source total differs for the same selector')
                    offset += len(items)
                    if offset >= total:
                        break
            print(f'{query} limit={limit} total={total} unique={len(seen)}')


def scan():
    baseline = '26d580e5'
    scanner = next((p.parent.parent / '.bin/jscpd'
                    for p in (Path.home() / '.npm/_npx').glob('*/node_modules/jscpd/package.json')
                    if json.loads(p.read_text()).get('version') == '4.0.5'), None)
    if scanner is None:
        raise ValueError('cached jscpd 4.0.5 is required')
    subprocess.run([scanner, '--version'], check=True)
    paths = subprocess.check_output(['git', 'diff', '--name-only', baseline, 'HEAD', '--',
                                    'crates/biomcp-adapters/src/openfda_label',
                                    'crates/biomcp-types/src/drug/query.rs'], text=True).splitlines()
    prior = []
    for name in paths:
        target = PROOF / 'baseline-source' / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(subprocess.check_output(['git', 'show', baseline + ':' + name]))
        prior.append(str(target))
    for label, selected in [('baseline', prior), ('current', paths)]:
        subprocess.run([scanner, *selected, '--format', 'rust', '--max-size', '10mb',
                        '--reporters', 'json', '--output', PROOF / f'jscpd-{label}'], check=True)
    subprocess.run(['ruff', 'check', '--select', 'F401,F841,ERA,BLE,E722,S110',
                    'scripts/label_search_proof.py'], check=True)


def retain():
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    destination = f'/home/ian/workspace/data/biomcp-redux-evidence/0605/{revision}'
    archive = PROOF / 'proof.tar.gz'
    with tarfile.open(archive, 'w:gz') as bundle:
        for item in sorted(PROOF.iterdir()):
            if item.name not in ['installed', 'proof.tar.gz']:
                bundle.add(item, arcname=item.name)
    subprocess.run(['ssh', 'yellow', 'mkdir', '-p', destination], check=True)
    subprocess.run(['scp', str(archive), 'yellow:' + destination + '/proof.tar.gz'], check=True)
    print('Evidence:', 'yellow:' + destination + '/proof.tar.gz')


if __name__ == '__main__':
    {'focused': focused, 'live': live, 'scan': scan, 'retain': retain}[sys.argv[1]]()

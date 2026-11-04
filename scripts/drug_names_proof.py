"""Reuse the source-bound public CLI for exact native-ID reply comparison."""
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import zipfile
import urllib.error
import urllib.parse
import urllib.request

PROOFS = Path('target/drug-names-proof')
ARCHIVE_SHA = 'c322e70ca7b9c3e5039740debf8575257d605d01f354261b28e121ac59eaf6f3'
BASELINE_SOURCE = 'd349f24892deac67c3b9bb836b708a487b5ea442'
BASELINE_BINARY_SHA = 'b50998e586ccc38a2a4f6d851cce229651e380560814e5d4212d4694f89c7660'


def extract():
    archive = PROOFS / 'baseline.zip'
    archive.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(['scp', 'yellow:/home/ian/workspace/data/biomcp-redux-evidence/0294/public-rehearsal-41806259796/native-macosx_11_0_arm64.zip', archive], check=True)
    if hashlib.sha256(archive.read_bytes()).hexdigest() != ARCHIVE_SHA:
        raise ValueError('public rehearsal archive differs from its receipt')
    with zipfile.ZipFile(archive) as bundle:
        with tarfile.open(fileobj=io.BytesIO(bundle.read('biomcp-darwin-arm64.tar.gz'))) as native:
            members = [m for m in native.getmembers() if m.isfile() and Path(m.name).name == 'biomcp']
            if len(members) != 1:
                raise ValueError('public native archive must contain one CLI')
            data = native.extractfile(members[0]).read()
    if hashlib.sha256(data).hexdigest() != BASELINE_BINARY_SHA:
        raise ValueError('public CLI differs from its source receipt')
    binary = PROOFS / 'baseline-bin/biomcp'
    binary.parent.mkdir(exist_ok=True)
    binary.write_bytes(data)
    binary.chmod(0o755)
    archive.unlink()
    print(f'baseline CLI {BASELINE_SOURCE} {BASELINE_BINARY_SHA}')


def native_replies(directory):
    replies = {}
    for path in directory.glob('*.json'):
        record = json.loads(path.read_text())
        if 'drug/name-get' in record.get('key', ''):
            continue
        rows = []
        for execution in record.get('executions', []):
            args = execution.get('args', [])
            if execution.get('surface') == 'cli' and any(args[i:i + 2] == ['get', 'drug'] for i in range(len(args) - 1)):
                rows.append((execution['exit'], bytes(execution['stdout'])))
        if rows:
            replies[path.name] = rows
    return replies


def compare():
    baseline = native_replies(PROOFS / 'baseline-transcripts')
    current = native_replies(PROOFS / 'transcripts')
    if not baseline or baseline.keys() != current.keys():
        raise ValueError('native-ID case inventory differs')
    changed = [name for name in baseline if baseline[name] != current[name]]
    if changed:
        raise ValueError(f'native-ID replies changed: {changed}')
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    binary = Path('target/drug-names-installed/bin/biomcp')
    report = {'source_commit': revision, 'binary_source_commit': revision,
              'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
              'baseline_source': BASELINE_SOURCE, 'baseline_binary_sha256': BASELINE_BINARY_SHA,
              'native_get_reply_count': sum(map(len, baseline.values())), 'native_get_byte_parity': True}
    (PROOFS / 'source-bindings.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))


def witness():
    destination = PROOFS / 'source-witness'
    destination.mkdir(parents=True, exist_ok=True)
    for suffix in ['.exact', '']:
        selector = f'products.brand_name{suffix}:"imatinib" OR products.active_ingredients.name{suffix}:"imatinib"'
        url = 'https://api.fda.gov/drug/drugsfda.json?' + urllib.parse.urlencode({'search': selector, 'limit': 50})
        try:
            with urllib.request.urlopen(url, timeout=30) as reply:
                body, status = reply.read(), reply.status
        except urllib.error.HTTPError as error:
            body, status = error.read(), error.code
        name = 'exact' if suffix else 'words'
        (destination / f'{name}.json').write_bytes(body)
        data = json.loads(body)
        facts = [{'application': row.get('application_number'),
                  'brands': [p.get('brand_name') for p in row.get('products', [])],
                  'ingredients': [i.get('name') for p in row.get('products', []) for i in p.get('active_ingredients', [])],
                  'openfda_names': {key: row.get('openfda', {}).get(key) for key in ['brand_name', 'generic_name', 'substance_name']}}
                 for row in data.get('results', [])]
        metadata = {'source': 'drugsfda', 'url': url, 'status': status,
                    'source_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                    'body_sha256': hashlib.sha256(body).hexdigest(), 'name_fields': facts}
        (destination / f'{name}-metadata.json').write_text(json.dumps(metadata, indent=2) + '\n')
        print(json.dumps(metadata, indent=2))


if __name__ == '__main__':
    {'extract': extract, 'compare': compare, 'witness': witness}[sys.argv[1]]()

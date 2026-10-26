"""Run the fixed approved cell-line plan once, retain evidence, and clean staging."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    root = Path.home() / 'workspace/experiments'
    root.mkdir(exist_ok=True)
    numbers = [int(p.name.split('-')[0]) for p in root.iterdir()
               if p.name.split('-')[0].isdigit()]
    folder = root / ('%03d-cell-line-captures' % (max(numbers + [0]) + 1))
    folder.mkdir()
    (folder / 'OWNER').write_text('biomcp-redux/0256\n')
    (folder / 'scope.json').write_text(json.dumps({
        'risk': 'Exact product request forms lack retained provider evidence',
        'rights': {'cellosaurus': 'CC BY 4.0', 'chembl': 'CC BY-SA 3.0'},
        'limits': 'Fixed seven rows once; no credentials, retry, redirect, fallback or extra page; stop whole plan at first failed outcome',
        'secondary': 'Deferred: nine retained cards contain primary accessions only',
        'zero_assay': 'No retained zero-total hint; use second exact accession',
    }, indent=2) + '\n')
    for name in ('cell_line_capture_plan.json', 'capture_cell_line_witnesses.py'):
        shutil.copyfile(Path('scripts') / name, folder / name)
    out = folder / 'native'
    with (folder / 'capture.log').open('wb') as log:
        result = subprocess.run(['make', 'source-capture',
                                'PLAN=' + str(folder / 'cell_line_capture_plan.json'),
                                'OUT=' + str(out)], stdout=log, stderr=subprocess.STDOUT)
    (folder / 'runner-exit.txt').write_text(str(result.returncode) + '\n')
    archive = Path(str(out) + '.tar.gz')
    subprocess.run(['python3', 'scripts/source_capture.py', '--verify', str(archive)], check=True)
    ledger = []
    for receipt in sorted(out.glob('*.receipt.json')):
        row = json.loads(receipt.read_text())
        prefix = receipt.name.split('.')[0]
        try:
            body = json.loads((out / (prefix + '.body')).read_text())
        except (ValueError, UnicodeError):
            body = {}
        rows = body.get('Cellosaurus', {}).get('cell-line-list', [])
        row['returned_accessions'] = [r.get('accession-list', []) for r in rows]
        row['native_count'] = body.get('page_meta', {}).get('total_count')
        row['paging'] = body.get('page_meta', 'Cellosaurus supplies no total or continuation')
        row['returned_cell_records'] = body.get('cell_lines', [])
        row['returned_rows'] = len(rows) if row['source'] == 'cellosaurus' else len(body.get('cell_lines', []))
        ledger.append(row)
    (folder / 'ledger.json').write_text(json.dumps(ledger, indent=2) + '\n')
    shutil.rmtree(out)
    manifest = {p.name: sha(p) for p in folder.iterdir() if p.is_file()}
    (folder / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    packed = folder.with_suffix('.tar.gz')
    with tarfile.open(packed, 'w:gz') as target:
        for path in sorted(folder.iterdir()):
            target.add(path, arcname=path.name, recursive=False)
    destination = '/home/ian/workspace/data/biomcp-redux/0256-cell-line-captures'
    subprocess.run(['ssh', 'yellow', 'mkdir', '-p', destination], check=True)
    subprocess.run(['scp', str(packed), 'yellow:' + destination + '/'], check=True)
    remote = destination + '/' + packed.name
    verify = '''import hashlib,json,tarfile,sys,io
p=sys.argv[1]
assert hashlib.sha256(open(p,'rb').read()).hexdigest()==sys.argv[2]
with tarfile.open(p) as t:
 names=t.getnames(); m=json.load(t.extractfile('manifest.json'))
 assert len(names)==len(set(names)) and set(names)==set(m)|{'manifest.json'}
 for n,h in m.items():
  assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
 raw=t.extractfile('native.tar.gz').read()
with tarfile.open(fileobj=io.BytesIO(raw)) as t:
 names=t.getnames(); m=json.load(t.extractfile('manifest.json'))
 assert len(names)==len(set(names)) and set(names)==set(m)|{'manifest.json'}
 for n,h in m.items():
  assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
print('Every outer and native member verified independently')
'''
    checksum = sha(packed)
    subprocess.run(['ssh', 'yellow', 'python3', '-', remote, checksum], input=verify.encode(), check=True)
    print(json.dumps({'archive': remote, 'sha256': checksum, 'ledger': ledger,
                      'capture_exit': result.returncode, 'removed_staging': [str(folder), str(packed)]}))
    shutil.rmtree(folder)
    packed.unlink()
    return 0


if __name__ == '__main__':
    raise SystemExit(main())

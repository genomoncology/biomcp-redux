"""Capture the three approved section witnesses once and retain them on Yellow."""
import hashlib
import json
import sys
from pathlib import Path
import subprocess
import tempfile
import time


def main():
    base = 'https://reactome.org/ContentService/data/'
    requests = [
        ('participants', 'participants/R-HSA-5673001'),
        ('events', 'pathway/R-HSA-5673001/containedEvents'),
        ('object-events', 'pathway/R-HSA-5673000/containedEvents'),
    ]
    if sys.argv[1:] == ['--object-only-control']:
        requests = [('object-only-control', 'pathway/R-HSA-6803211/containedEvents')]
    elif sys.argv[1:]:
        raise ValueError('only --object-only-control is accepted; no retry or fallback')
    plan = {'deadline_seconds': 8, 'max_bytes': 8 * 1024 * 1024,
            'spacing_seconds': 1, 'requests': [
                {'id': name, 'source': 'reactome', 'url': base + path,
                 'response_format': 'json'} for name, path in requests]}
    parent = Path.home() / 'workspace/job-evidence/biomcp-redux-0219'
    parent.mkdir(parents=True, exist_ok=True)
    out = parent / ('native-' + str(time.time_ns()))
    with tempfile.TemporaryDirectory() as scratch:
        plan_path = Path(scratch) / 'plan.json'
        plan_path.write_text(json.dumps(plan))
        subprocess.run(['make', 'source-capture', 'PLAN=' + str(plan_path),
                        'OUT=' + str(out)], check=True)
    archive = Path(str(out) + '.tar.gz')
    subprocess.run(['python3', 'scripts/source_capture.py', '--verify', str(archive)], check=True)
    checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
    destination = '/home/ian/workspace/data/biomcp-redux/0219-reactome-sections'
    subprocess.run(['ssh', 'yellow', 'mkdir', '-p', destination], check=True)
    subprocess.run(['scp', str(archive), 'yellow:' + destination + '/'], check=True)
    remote = subprocess.check_output(['ssh', 'yellow', 'sha256sum',
                                      destination + '/' + archive.name], text=True)
    if remote.split()[0] != checksum:
        raise ValueError('destination archive hash mismatch')
    for index, (name, _) in enumerate(requests):
        rows = json.loads((out / f'{index:04}.body').read_text())
        print(name, 'rows', len(rows), 'integers',
              sum(isinstance(row, int) for row in rows))
    print('Retained archive:', destination + '/' + archive.name, checksum)


if __name__ == '__main__':
    main()

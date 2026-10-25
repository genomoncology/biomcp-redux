"""Run the bounded public diagnostic once and retain verified evidence before cleanup."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tarfile
import tempfile

COMMAND = ['cargo', 'test', '--locked', '-p', 'biomcp', '--test', 'cases',
           '--', '--live-check', '--details']
FILES = ('report.json', 'build.log', 'exit-status', 'run.json')

# The receiver creates a unique destination, accepts only our named plain files,
# and verifies the sender's manifest before acknowledging durable delivery.
RECEIVE = r'''
import hashlib, json, os, pathlib, sys, tarfile
root = pathlib.Path(sys.argv[1])
root.mkdir(parents=True, exist_ok=False)
allowed = {'report.json', 'build.log', 'exit-status', 'run.json', 'manifest.json'}
seen = set()
with tarfile.open(fileobj=sys.stdin.buffer, mode='r|') as archive:
    for member in archive:
        if member.name not in allowed or member.name in seen or not member.isfile():
            raise ValueError('unexpected evidence entry')
        seen.add(member.name)
        source = archive.extractfile(member)
        with (root / member.name).open('xb') as target:
            while True:
                chunk = source.read(1024 * 1024)
                if not chunk:
                    break
                target.write(chunk)
            target.flush()
            os.fsync(target.fileno())
if seen != allowed:
    raise ValueError('incomplete evidence')
manifest_bytes = (root / 'manifest.json').read_bytes()
if hashlib.sha256(manifest_bytes).hexdigest() != sys.argv[2]:
    raise ValueError('manifest transfer mismatch')
manifest = json.loads(manifest_bytes)
if set(manifest) != allowed - {'manifest.json'}:
    raise ValueError('unexpected manifest entries')
for name, expected in manifest.items():
    digest = hashlib.sha256()
    with (root / name).open('rb') as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b''):
            digest.update(chunk)
    if digest.hexdigest() != expected:
        raise ValueError('evidence checksum mismatch: ' + name)
fd = os.open(str(root), os.O_RDONLY)
try:
    os.fsync(fd)
finally:
    os.close(fd)
'''


def digest(path):
    checksum = hashlib.sha256()
    with path.open('rb') as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b''):
            checksum.update(chunk)
    return checksum.hexdigest()


def run_check(scratch):
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    (scratch / 'run.json').write_text(json.dumps({
        'revision': revision, 'command': COMMAND,
        'started': datetime.datetime.now(datetime.timezone.utc).isoformat()}) + '\n')
    with (scratch / 'report.json').open('wb') as report, (scratch / 'build.log').open('wb') as log:
        try:
            result = subprocess.run(COMMAND, stdout=report, stderr=log, check=False).returncode
        except OSError as error:
            log.write(str(error).encode())
            result = 127
    (scratch / 'exit-status').write_text(str(result) + '\n')
    try:
        report = json.loads((scratch / 'report.json').read_text())
        counts = [report['drift_cases'], report['error_cases']]
        if any(type(count) is not int or count < 0 for count in counts):
            raise ValueError('invalid drift/error counts')
        return result != 0 or any(counts)
    except (ValueError, KeyError, TypeError) as error:
        print('Invalid live-check report: ' + str(error), file=sys.stderr)
        return True


def deliver(scratch, store):
    manifest = {name: digest(scratch / name) for name in FILES}
    (scratch / 'manifest.json').write_text(json.dumps(manifest, sort_keys=True) + '\n')
    archive = scratch / 'transfer.tar'
    with tarfile.open(archive, 'w') as target:
        for name in (*FILES, 'manifest.json'):
            target.add(scratch / name, arcname=name, recursive=False)
    destination = Path(store['workspaceRoot']) / 'data/biomcp-redux/0068-scheduled' / scratch.name
    command = shlex.join(['python3', '-c', RECEIVE, str(destination),
                          digest(scratch / 'manifest.json')])
    with archive.open('rb') as source:
        subprocess.run(['ssh', '-o', 'BatchMode=yes', '-o', 'ConnectTimeout=30',
                        store['names'][0], command], stdin=source, check=True)
    print('Verified evidence: ' + store['names'][0] + ':' + str(destination))


def main():
    workspace = Path.home() / 'workspace'
    machines = json.loads((workspace / 'repos/agents/machines.json').read_text())
    scratch_root = workspace / 'data/biomcp-redux/0068-scheduled-scratch'
    scratch_root.mkdir(parents=True, exist_ok=True)
    scratch = Path(tempfile.mkdtemp(prefix=datetime.datetime.now(
        datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ-'), dir=scratch_root))
    print('Owned scratch: ' + str(scratch), flush=True)
    try:
        failed = run_check(scratch)
        deliver(scratch, machines['yellow'])
    except (OSError, ValueError, KeyError, subprocess.SubprocessError) as error:
        print('Nightly check failed; retained scratch: ' + str(scratch) + ': ' + str(error),
              file=sys.stderr)
        return 1
    shutil.rmtree(scratch)
    return int(bool(failed))


if __name__ == '__main__':
    sys.exit(main())

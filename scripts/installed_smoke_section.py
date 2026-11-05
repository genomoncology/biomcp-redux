"""Run the installed smoke section using retained release bytes, without building."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import threading
import time

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from release.artifacts import inspect


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def identities(pid):
    """Read only descendants' identity and scheduling metadata, never arguments."""
    found = []
    pending = [pid]
    while pending:
        current = pending.pop()
        root = Path('/proc') / str(current)
        try:
            for task in (root / 'task').iterdir():
                pending.extend(int(value) for value in (task / 'children').read_text().split())
            status = dict(line.split(':', 1) for line in (root / 'status').read_text().splitlines())
            found.append({'pid': current, 'ppid': int(status['PPid']),
                          'executable': (root / 'exe').readlink().name,
                          'state': status['State'].strip()})
        except (FileNotFoundError, ProcessLookupError):
            continue
    return found


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('artifacts', type=Path)
    parser.add_argument('proof', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('--candidate', required=True)
    args = parser.parse_args()
    artifacts, proof = args.artifacts.resolve(), args.proof.resolve()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    wheel, = artifacts.glob('*.whl')
    record = inspect(wheel)
    bootstrap = json.loads((proof / 'bootstrap.json').read_text())
    if record['revision'] != args.candidate or bootstrap['product_revision'] != args.candidate or bootstrap['tooling_revision'] != args.candidate:
        raise ValueError('artifact and runner source must match the selected candidate')
    installed = output / 'installed'
    subprocess.run([sys.executable, '-m', 'venv', str(installed)], check=True)
    subprocess.run([str(installed / 'bin/python'), '-m', 'pip', 'install', '--no-index', '--no-deps', str(wheel)], check=True)
    binary = installed / 'bin/biomcp'
    if digest(binary) != record['binary_sha256']:
        raise ValueError('installed binary differs from release manifest')
    fixture_tree = subprocess.check_output(['git', 'ls-tree', '-r', args.candidate, '--', 'tests', 'sdlc/planning/slices/gene.md'], text=True)
    actual_tree = subprocess.check_output(['git', 'ls-tree', '-r', 'HEAD', '--', 'tests', 'sdlc/planning/slices/gene.md'], text=True)
    if fixture_tree != actual_tree:
        raise ValueError('smoke fixtures differ from the selected candidate')
    (output / 'fixture-source.txt').write_text(fixture_tree)
    for name in ['case-runner', 'tool-runner']:
        listing = subprocess.check_output([str(proof / name), '--list'], env=dict(os.environ, BIOMCP_TEST_FIXTURES=str(proof / 'fixtures')), text=True)
        (output / (name + '-list.txt')).write_text(listing)
    binding = {'candidate': args.candidate, 'product': record, 'bootstrap': bootstrap,
               'fixture_source_sha256': digest(output / 'fixture-source.txt'),
               'wheel_sha256': digest(wheel), 'binary_sha256': digest(binary),
               'runners': {name: digest(proof / name) for name in ['case-runner', 'tool-runner']},
               'tooling_revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
               'toolchain': json.loads(Path('release/toolchain.json').read_text()),
               'source_files': {str(path): digest(path) for path in [Path(__file__), Path('release/smoke.py'), Path('release/test_smoke.py')]}}
    (output / 'binding.json').write_text(json.dumps(binding, indent=2) + '\n')
    env = dict(os.environ, BIOMCP_INSTALLED_BINARY=str(binary), BIOMCP_CASE_RUNNER=str(proof / 'case-runner'),
               BIOMCP_INSTALLED_VERSION=record['version'], BIOMCP_EVIDENCE_DIR=str(output / 'fixture-evidence'),
               BIOMCP_SMOKE_PHASE_FILE=str(output / 'phases.jsonl'))
    command = [sys.executable, '-m', 'unittest', '-v', 'release.test_smoke']
    stopped = threading.Event()
    with (output / 'stdout.log').open('wb') as stdout, (output / 'stderr.log').open('wb') as stderr:
        child = subprocess.Popen(command, env=env, stdout=stdout, stderr=stderr, start_new_session=True)
        def observe():
            with (output / 'processes.jsonl').open('a') as log:
                while not stopped.is_set():
                    log.write(json.dumps({'monotonic_ns': time.monotonic_ns(), 'processes': identities(child.pid)}) + '\n')
                    log.flush()
                    stopped.wait(20)
        observer = threading.Thread(target=observe)
        observer.start()
        try:
            status = child.wait()
        finally:
            stopped.set()
            observer.join()
            try:
                os.killpg(child.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
    (output / 'result.json').write_text(json.dumps({'exit': status, 'command': command}) + '\n')
    print(json.dumps({'exit': status, 'output': str(output)}), flush=True)
    return status


if __name__ == '__main__':
    sys.exit(main())

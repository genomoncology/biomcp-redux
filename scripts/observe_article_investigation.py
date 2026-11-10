"""Observe one keyless article command using a separately retained executable."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

from nightly_live_check import PASSED_ENVIRONMENT


def main():
    source, expected, destination = sys.argv[1:]
    root = Path(os.environ['TMPDIR']) / 'article-observation'
    root.mkdir()
    binary = root / 'biomcp'
    subprocess.run(['scp', '-q', source, str(binary)], check=True)
    actual = hashlib.file_digest(binary.open('rb'), 'sha256').hexdigest()
    if actual != expected:
        raise ValueError('retained executable checksum differs')
    binary.chmod(0o755)
    home = root / 'home'
    home.mkdir()
    environment = {key: os.environ[key] for key in PASSED_ENVIRONMENT if key in os.environ}
    environment.update(HOME=str(home), XDG_CACHE_HOME=str(home / 'cache'),
                       XDG_CONFIG_HOME=str(home / 'config'))
    observations = root / 'observations.jsonl'
    command = [str(binary), '--observe', str(observations), '--format', 'json',
               'search', 'article', 'AKT1 E17K']
    started = time.monotonic()
    reply = subprocess.run(command, capture_output=True, timeout=90, env=environment, check=False)
    (root / 'article.stdout.json').write_bytes(reply.stdout)
    (root / 'article.stderr.txt').write_bytes(reply.stderr)
    metadata = {'product_revision': '972c98e98f483d52a7cbd76996a6db422282dd5d',
                'product_sha256': actual,
                'driver_revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                'elapsed_ms': round((time.monotonic() - started) * 1000),
                'exit_code': reply.returncode, 'attempts': 1}
    (root / 'metadata.json').write_text(json.dumps(metadata, indent=2) + '\n')
    payloads = ['article.stdout.json', 'article.stderr.txt', 'observations.jsonl', 'metadata.json']
    manifest = ''.join(hashlib.sha256((root / name).read_bytes()).hexdigest() + '  ' + name + '\n'
                       for name in payloads)
    (root / 'SHA256SUMS').write_text(manifest)
    host, directory = destination.split(':', 1)
    subprocess.run(['ssh', host, 'mkdir', '-p', directory], check=True)
    for name in payloads + ['SHA256SUMS']:
        subprocess.run(['scp', '-q', str(root / name), destination + '/' + name], check=True)
    verification = ('from pathlib import Path\nimport hashlib\np=Path(' + repr(directory) + ')\n'
                    'for row in (p/"SHA256SUMS").read_text().splitlines():\n'
                    ' h,n=row.split("  ",1)\n'
                    ' assert hashlib.sha256((p/n).read_bytes()).hexdigest()==h,n\n'
                    'print("receiver verified four observation payloads")\n')
    subprocess.run(['ssh', host, 'python3', '-'], input=verification, text=True, check=True)
    print(json.dumps(metadata), flush=True)
    print('manifest_sha256', hashlib.sha256(manifest.encode()).hexdigest(), flush=True)
    for row in observations.read_text().splitlines():
        value = json.loads(row)
        print(json.dumps({key: value.get(key) for key in
                          ['source', 'url', 'status', 'cached', 'elapsed_ms', 'tls_cause']}), flush=True)
    envelope = json.loads(reply.stdout)
    print(json.dumps({'status': envelope.get('status'), 'sources': envelope.get('sources')}), flush=True)


if __name__ == '__main__':
    main()

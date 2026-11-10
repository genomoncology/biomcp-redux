"""One-off post-landing checks for the retained 972c98e binary."""
import hashlib
import os
from pathlib import Path
import subprocess
import sys
import tempfile

from nightly_live_check import PASSED_ENVIRONMENT, check

SOURCE = 'yellow:workspace/data/biomcp-redux-evidence/0785-0798/core-batch-20261110/gate-j36792-artifact/biomcp'
EXPECTED = '0fa14f7fc0aba554d9b2b759b482732a8d476eae555a6ff66d495dfccf966fa8'
PROOFS = ['mcp-parity/typed-cli-bridge', 'mcp-parity/reply-forms',
          'mcp-parity/typed-followups', 'mcp-hosted/use/header-boundary',
          'mcp-hosted/use/continuation-rights', 'mcp-parity/cancel-preserves-another-active-call']
CHECKS = [('search', 'article', 'AKT1 E17K'), ('search', 'cell-line', 'MOLM13'),
          ('get', 'cell-line', 'CVCL_0004', 'identity', '--section-limit', '2')]


def run(command):
    print('RUN', command, flush=True)
    result = subprocess.run(command, check=False)
    print('RESULT', result.returncode, command, flush=True)
    return result.returncode


def main():
    os.environ['RUSTUP_TOOLCHAIN'] = '1.95.0'
    binary = Path(tempfile.gettempdir()) / 'biomcp-ending-0788'
    subprocess.run(['scp', '-q', SOURCE, str(binary)], check=True)
    actual = hashlib.file_digest(binary.open('rb'), 'sha256').hexdigest()
    print('PRODUCT 972c98e98f483d52a7cbd76996a6db422282dd5d sha256', actual, flush=True)
    if actual != EXPECTED:
        return 1
    binary.chmod(0o755)
    status = 0
    cmds = [
        ['python3.13', 'scripts/trials.py', '--target', 'tools', '--proofs', '--list'],
        ['python3.13', 'scripts/trials.py', '--target', 'tools', '--proofs', '--installed', '--executable', str(binary), *PROOFS],
        ['cargo', 'test', '--locked', '-p', 'biomcp-query', '--test', 'health'],
        ['cargo', 'test', '--locked', '-p', 'biomcp-fetch', '--test', 'transport', 'cancellation'],
        ['cargo', 'test', '--locked', '-p', 'biomcp-fetch', '--test', 'cache', 'credentials'],
        ['make', 'docs-check'],
    ]
    for command in cmds:
        status |= bool(run(command))
    with tempfile.TemporaryDirectory(prefix='biomcp-live-0788-') as home:
        environment = {key: os.environ[key] for key in PASSED_ENVIRONMENT if key in os.environ}
        environment.update(HOME=home, XDG_CACHE_HOME=home + '/cache', XDG_CONFIG_HOME=home + '/config')
        for args in CHECKS:
            problem = check(str(binary), args, environment, 120, 1)
            print('LIVE RESULT', args, 'ok' if problem is None else 'failed', flush=True)
            status |= problem is not None
    return int(status)


if __name__ == '__main__':
    sys.exit(main())

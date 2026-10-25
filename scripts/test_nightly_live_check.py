"""Exercise the scheduled command offline, with fake Cargo and a local SSH store."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
CARGO = '''#!/usr/bin/env python3
import json, os, sys
from pathlib import Path
assert sys.argv[1:] == ['test','--locked','-p','biomcp','--test','cases','--','--live-check','--details']
print(json.dumps(json.loads(os.environ['FAKE_REPORT'])))
print('offline build log', file=sys.stderr)
sys.exit(int(os.environ.get('FAKE_EXIT', '0')))
'''
SSH = '''#!/usr/bin/env python3
import os, subprocess, sys
if os.environ.get('FAKE_COPY') == 'fail':
    sys.exit(23)
result = subprocess.run(sys.argv[2:], shell=False, input=sys.stdin.buffer.read()) if False else subprocess.run(sys.argv[2], shell=True, input=sys.stdin.buffer.read())
if os.environ.get('FAKE_COPY') == 'manifest':
    sys.exit(24)
sys.exit(result.returncode)
'''


class NightlyCommand(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.checkout = self.root / 'checkout'
        (self.checkout / 'scripts').mkdir(parents=True)
        shutil.copy(ROOT / 'Makefile', self.checkout / 'Makefile')
        script = ROOT / 'scripts/nightly_live_check.py'
        if script.exists():
            shutil.copy(script, self.checkout / 'scripts')
        self.home = self.root / 'home'
        self.workspace = self.home / 'workspace'
        agents = self.workspace / 'repos/agents'
        agents.mkdir(parents=True)
        self.store = self.root / 'store'
        (agents / 'machines.json').write_text(json.dumps({
            'yellow': {'names': ['offline-store'], 'workspaceRoot': str(self.store)}}))
        self.tools = self.root / 'tools'
        self.tools.mkdir()
        for name, content in [('cargo', CARGO), ('ssh', SSH)]:
            target = self.tools / name
            target.write_text(content)
            target.chmod(0o755)
        self.env = dict(os.environ, HOME=str(self.home),
                        TMPDIR=str(self.root / 'tmp'),
                        PATH=str(self.tools) + os.pathsep + os.environ['PATH'],
                        FAKE_REPORT='{"drift_cases":0,"error_cases":0}')
        Path(self.env['TMPDIR']).mkdir()

    def run_command(self, **values):
        env = dict(self.env, **values)
        return subprocess.run(['make', 'nightly-live-check'], cwd=self.checkout,
                              env=env, text=True, capture_output=True, check=False)

    def copied(self):
        runs = list((self.store / 'data/biomcp-redux/0068-scheduled').glob('*'))
        self.assertEqual(len(runs), 1)
        run = runs[0]
        manifest = json.loads((run / 'manifest.json').read_text())
        for name, digest in manifest.items():
            self.assertEqual(hashlib.sha256((run / name).read_bytes()).hexdigest(), digest)
        self.assertEqual((run / 'build.log').read_text(), 'offline build log\n')
        return run

    def scratch(self):
        return list((self.workspace / 'data/biomcp-redux/0068-scheduled-scratch').glob('*'))

    def test_clean_copies_and_cleans_only_owned_scratch(self):
        result = self.run_command()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual((self.copied() / 'exit-status').read_text(), '0\n')
        self.assertEqual(self.scratch(), [])
        second = self.run_command()
        self.assertEqual(second.returncode, 0, second.stderr)
        self.assertEqual(len(list((self.store / 'data/biomcp-redux/0068-scheduled').glob('*'))), 2)

    def test_drift_errors_and_failed_runner_still_copy(self):
        for overrides in [dict(FAKE_REPORT='{"drift_cases":1,"error_cases":0}'),
                          dict(FAKE_REPORT='{"drift_cases":0,"error_cases":1}'),
                          dict(FAKE_EXIT='7'), dict(FAKE_REPORT='{}')]:
            with self.subTest(overrides=overrides):
                result = self.run_command(**overrides)
                self.assertNotEqual(result.returncode, 0)
                self.copied()
                self.assertEqual(self.scratch(), [])
                shutil.rmtree(self.store)

    def test_failed_copy_or_manifest_keeps_scratch(self):
        for failure in ['fail', 'manifest']:
            with self.subTest(failure=failure):
                result = self.run_command(FAKE_COPY=failure)
                self.assertNotEqual(result.returncode, 0)
                scratch = self.scratch()
                self.assertEqual(len(scratch), 1, result.stdout + result.stderr)
                self.assertTrue((scratch[0] / 'report.json').is_file())
                self.assertTrue((scratch[0] / 'manifest.json').is_file())
                shutil.rmtree(scratch[0])


if __name__ == '__main__':
    unittest.main()

"""Exercise the source capture command against an offline HTTP source."""
import hashlib
import http.server
import json
import os
from pathlib import Path
import subprocess
import tempfile
import threading
import unittest

SCRIPT = Path(__file__).with_name('source_capture.py').resolve()
ROOT = SCRIPT.parent.parent


class Source(http.server.BaseHTTPRequestHandler):
    calls = []

    def do_GET(self):
        self.server.calls.append(self.path)
        status = 429 if self.path == '/stop' else 200
        self.send_response(status)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        self.wfile.write(b'{"rows":[]}')

    def log_message(self, *_args):
        pass


class CapturePlanTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Source)
        self.server.calls = []
        thread = threading.Thread(target=self.server.serve_forever)
        thread.start()
        self.addCleanup(thread.join)
        self.addCleanup(self.server.server_close)
        self.addCleanup(self.server.shutdown)
        self.env = dict(os.environ, TMPDIR=str(self.base))

    def plan(self, paths):
        url = 'http://127.0.0.1:' + str(self.server.server_port)
        value = {'deadline_seconds': 2, 'max_bytes': 4096, 'spacing_seconds': 0,
                 'requests': [{'id': str(n), 'source': 'stub', 'url': url + path,
                               'response_format': 'json'} for n, path in enumerate(paths)]}
        path = self.base / 'plan.json'
        path.write_text(json.dumps(value))
        return path

    def run_capture(self, plan, out, *args, env=None):
        return subprocess.run(['python3', str(SCRIPT), '--plan', str(plan),
                               '--out', str(out), *args], env=env or self.env,
                              capture_output=True, text=True)

    def test_full_run_retains_and_verifies_archive_before_cleanup(self):
        out = self.base / 'capture'
        result = self.run_capture(self.plan(['/one', '/two']), out)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.server.calls, ['/one', '/two'])
        self.assertEqual((out / '0000.body').read_bytes(), b'{"rows":[]}')
        archive = out.with_suffix('.tar.gz')
        result = subprocess.run(['python3', str(SCRIPT), '--verify', str(archive)],
                                env=self.env, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(out.exists())

    def test_rate_limit_stops_and_continuation_sends_only_undelivered(self):
        plan = self.plan(['/one', '/stop', '/three'])
        first, second = self.base / 'first', self.base / 'second'
        result = self.run_capture(plan, first)
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertEqual(self.server.calls, ['/one', '/stop'])
        result = self.run_capture(plan, second, '--continue-from', str(first))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.server.calls, ['/one', '/stop', '/three'])
        receipt = json.loads((second / '0001.receipt.json').read_text())
        self.assertEqual(receipt['status'], 429)

    def test_preflight_refuses_existing_output_and_missing_os_variable(self):
        plan = self.plan(['/one'])
        for name, env in [('existing', self.env),
                          ('missing', {k: v for k, v in self.env.items() if k != 'TMPDIR'})]:
            with self.subTest(name=name):
                out = self.base / name
                if name == 'existing':
                    out.mkdir()
                result = self.run_capture(plan, out, env=env)
                self.assertEqual(result.returncode, 1)
                self.assertEqual(self.server.calls, [])

    def test_manifest_rejects_one_changed_byte_before_continuation(self):
        plan = self.plan(['/one'])
        first = self.base / 'first'
        self.assertEqual(self.run_capture(plan, first).returncode, 0)
        (first / '0000.body').write_bytes(b'{"rows":[1]}')
        result = self.run_capture(plan, self.base / 'second', '--continue-from', str(first))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(self.server.calls, ['/one'])

    def test_make_target_runs_one_request(self):
        result = subprocess.run(['make', 'source-capture', 'PLAN=' + str(self.plan(['/make'])),
                                 'OUT=' + str(self.base / 'make')], cwd=ROOT,
                                env=self.env, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.server.calls, ['/make'])


if __name__ == '__main__':
    unittest.main()

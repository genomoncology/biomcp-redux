"""Offline tests of the retained recorder's actual request and comparison path."""
import importlib.util
from pathlib import Path
import tempfile
import threading
import urllib.request
import urllib.error
import unittest
from unittest.mock import patch


def recorder():
    spec = importlib.util.spec_from_file_location(
        'capture_recipe', Path(__file__).with_name('capture_baseline.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class Reply:
    def __init__(self, status, body):
        self.status, self.body = status, body
        self.headers = {'Content-Type': 'application/json'}
    def __enter__(self): return self
    def __exit__(self, *args): pass
    def read(self, limit): return self.body[:limit]


def capture(recipe, replies, refusal=None, path='/drug/label.json'):
    with tempfile.TemporaryDirectory() as directory:
        recipe.OUT = Path(directory)
        (recipe.OUT / 'captures').mkdir()
        recipe.LOCK = threading.RLock()
        recipe.PHYSICAL, recipe.LOCAL = [], []
        recipe.CACHE, recipe.FAILED = {}, set()
        recipe.ALLOWED, recipe.CHEMBL = set(), set()
        recipe.CURRENT = {'slice': 'drug', 'index': 1}
        recipe.CAPS = {('drug', 'openfda'): 0 if refusal == 'budget' else 16}
        recipe.COUNTS = {('drug', 'openfda'): 0}
        recipe.BASES = {'openfda': 'https://api.fda.gov'}
        server = recipe.Server(('127.0.0.1', 0), recipe.Handler)
        recipe.PORT = server.server_port
        thread = threading.Thread(target=server.serve_forever)
        thread.start()
        opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
        try:
            with patch.object(recipe.OPENER, 'open') as upstream:
                for number, reply in enumerate(replies):
                    url = 'https://api.fda.gov' + path + '?n=' + str(number)
                    if refusal != 'identity': recipe.declaration('GET', url, None)
                    if isinstance(reply, Exception): upstream.side_effect = reply
                    else: upstream.return_value = Reply(*reply)
                    try:
                        with opener.open('http://127.0.0.1:' + str(recipe.PORT)
                                         + '/openfda' + path + '?n=' + str(number)) as response:
                            response.read()
                    except urllib.error.HTTPError as error:
                        error.read()
                calls = upstream.call_count
        finally:
            server.shutdown()
            server.server_close()
            thread.join()
        return recipe.LOCAL, calls


class CaptureTests(unittest.TestCase):
    def test_declared_empty_allows_next_distinct_request(self):
        recipe = recorder()
        rows, calls = capture(recipe, [
            (404, b'{"error":{"code":"NOT_FOUND","message":"No matches found!"}}'),
            (200, b'{"results":[{"text":"429"}]}'),
        ])
        self.assertEqual([r['outcome'] for r in rows], ['empty', 'success'])
        self.assertTrue(recipe.comparison_eligible(0, rows))
        self.assertFalse(recipe.FAILED)
        self.assertEqual(calls, 2)

    def test_outcomes_control_stop_and_eligibility(self):
        scenarios = [
            (200, b'{"score":0.429}', {}, 'success', True),
            (404, b'{"error":{"code":"NOT_FOUND","message":"No matches found!"}}', {}, 'empty', True),
            (404, b'{"error":"route missing"}', {}, 'provider_failure', False),
            (429, b'{"error":{"code":"TOO_MANY_REQUESTS"}}', {}, 'provider_failure', False),
            (200, b'{"results":', {}, 'malformed', False),
            (200, b'{"results":[]}', {'truncated': True}, 'truncated', False),
            (502, b'{}', {'transport_failure': True}, 'transport_failure', False),
            (409, b'{}', {'refusal': 'budget_exhausted'}, 'budget_refusal', False),
            (409, b'{}', {'refusal': 'undeclared_identity'}, 'identity_refusal', False),
        ]
        recipe = recorder()
        for status, body, flags, expected, eligible in scenarios:
            with self.subTest(expected=expected):
                result = recipe.classify('openfda', 'https://api.fda.gov/drug/label.json', status, body, **flags)
                self.assertEqual(result.kind.value, expected)
                self.assertEqual(result.stop_source, not eligible)
                self.assertEqual(recipe.comparison_eligible(0, [{'outcome': result.kind.value}]), eligible)
                response = (status, body)
                refusal = None
                if flags.get('transport_failure'):
                    response = urllib.error.URLError('synthetic transport failure')
                if flags.get('truncated'):
                    response = (status, b' ' * (8 * 1024 * 1024 + 1))
                if flags.get('refusal'):
                    refusal = 'budget' if expected == 'budget_refusal' else 'identity'
                rows, calls = capture(recipe, [response], refusal)
                self.assertEqual(rows[0]['outcome'], expected)
                self.assertEqual(recipe.comparison_eligible(0, rows), eligible)
                self.assertEqual(bool(recipe.FAILED), not eligible)
                self.assertEqual(calls, 0 if refusal else 1)
        unknown = recipe.classify('openfda', 'https://api.fda.gov/unknown', 404,
            b'{"error":{"code":"NOT_FOUND","message":"No matches found!"}}')
        self.assertEqual(unknown.kind.value, 'provider_failure')
        rows, calls = capture(recipe, [(404, b'{"error":{"code":"NOT_FOUND","message":"No matches found!"}}')], path='/unknown')
        self.assertEqual(rows[0]['outcome'], 'provider_failure')
        self.assertTrue(recipe.FAILED)


if __name__ == '__main__':
    unittest.main()

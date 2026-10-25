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


def capture(recipe, replies, refusal=None, path='/drug/label.json',
            source='openfda', host='api.fda.gov', repeat=False, method='GET'):
    with tempfile.TemporaryDirectory() as directory:
        recipe.OUT = Path(directory)
        (recipe.OUT / 'captures').mkdir()
        recipe.LOCK = threading.RLock()
        recipe.PHYSICAL, recipe.LOCAL = [], []
        recipe.CACHE, recipe.FAILED = {}, set()
        recipe.ALLOWED, recipe.CHEMBL = set(), set()
        recipe.CURRENT = {'slice': 'drug', 'index': 1}
        recipe.CAPS = {('drug', source): 0 if refusal == 'budget' else 16}
        recipe.COUNTS = {('drug', source): 0}
        recipe.BASES = {source: 'https://' + host}
        server = recipe.Server(('127.0.0.1', 0), recipe.Handler)
        recipe.PORT = server.server_port
        thread = threading.Thread(target=server.serve_forever)
        thread.start()
        opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
        try:
            with patch.object(recipe.OPENER, 'open') as upstream:
                for number, reply in enumerate(replies):
                    query = '?n=' + str(0 if repeat else number)
                    url = 'https://' + host + path + query
                    body = {} if method == 'POST' else None
                    if refusal != 'identity': recipe.declaration(method, url, body)
                    if isinstance(reply, Exception): upstream.side_effect = reply
                    else: upstream.return_value = Reply(*reply)
                    try:
                        request = urllib.request.Request(
                            'http://127.0.0.1:' + str(recipe.PORT) + '/' + source + path + query,
                            data=b'{}' if method == 'POST' else None, method=method)
                        with opener.open(request) as response:
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
    def test_graphql_errors_stop_forwarding_and_cannot_be_reused(self):
        for source, host, path in [
            ('civic', 'civicdb.org', '/api/graphql'),
            ('opentargets', 'api.platform.opentargets.org', '/api/v4/graphql'),
        ]:
            with self.subTest(source=source):
                recipe = recorder()
                rows, calls = capture(recipe, [
                    (200, b'{"data":null,"errors":[{"message":"unavailable"}]}'),
                    (200, b'{"data":{}}'),
                ], source=source, host=host, path=path, repeat=True, method='POST')
                self.assertEqual([r['outcome'] for r in rows],
                                 ['provider_failure', 'local_refusal'])
                self.assertFalse(recipe.comparison_eligible(0, rows))
                self.assertFalse(recipe.CACHE)
                self.assertEqual(calls, 1)

    def test_exact_empty_reply_is_reused_without_another_provider_call(self):
        recipe = recorder()
        rows, calls = capture(recipe, [
            (404, b'{"error":{"code":"NOT_FOUND","message":"No matches found!"}}'),
            (500, b'{"error":"must not be forwarded"}'),
        ], repeat=True)
        self.assertEqual([r['outcome'] for r in rows], ['empty', 'empty'])
        self.assertTrue(rows[1]['reused_retained_body'])
        self.assertTrue(recipe.comparison_eligible(0, rows))
        self.assertEqual(calls, 1)

    def test_graphql_error_field_requires_the_exact_native_endpoint(self):
        recipe = recorder()
        for source, url, body in [
            ('civic', 'https://civicdb.org/api/graphql', b'{"data":{},"errors":[]}'),
            ('civic', 'https://civicdb.org/other', b'{"errors":["a normal field"]}'),
            ('civic', 'https://other.invalid/api/graphql', b'{"errors":["a normal field"]}'),
            ('other', 'https://civicdb.org/api/graphql', b'{"errors":["a normal field"]}'),
        ]:
            with self.subTest(source=source, url=url):
                self.assertEqual(recipe.classify(source, url, 200, body).kind.value, 'success')

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

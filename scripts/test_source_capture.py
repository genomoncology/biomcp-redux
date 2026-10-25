"""Offline tests of the retained recorder's actual request and comparison path."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


def recorder():
    spec = importlib.util.spec_from_file_location(
        'capture_recipe', Path(__file__).with_name('capture_baseline.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class CaptureTests(unittest.TestCase):
    def test_declared_empty_allows_next_distinct_request(self):
        recipe = recorder()
        rows = recipe.offline_capture_sequence([
            (404, b'{"error":{"code":"NOT_FOUND","message":"No matches found!"}}'),
            (200, b'{"results":[{"text":"429"}]}'),
        ])
        self.assertEqual([r['outcome'] for r in rows], ['empty', 'success'])
        self.assertTrue(recipe.comparison_eligible(0, rows))
        self.assertFalse(recipe.FAILED)

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
        unknown = recipe.classify('openfda', 'https://api.fda.gov/unknown', 404,
            b'{"error":{"code":"NOT_FOUND","message":"No matches found!"}}')
        self.assertEqual(unknown.kind.value, 'provider_failure')


if __name__ == '__main__':
    unittest.main()

"""Probe contracts exercise no real credentials, network or remote writes."""
import contextlib
import io
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from release import channel_credentials as probe

SECRET = 'credential-sentinel-do-not-disclose'
LOCATION = probe.UPLOAD + 'opaque-upload?state=opaque%2Fvalue&other=kept'
ENV = {'PATH': os.environ['PATH'], 'GITHUB_ACTIONS': 'true',
       'GITHUB_REPOSITORY': probe.REPOSITORY, 'GITHUB_EVENT_NAME': 'workflow_dispatch',
       'GITHUB_REF': 'refs/heads/credential-probe-0807-abcdef012345',
       'GITHUB_SHA': 'a' * 40, 'GITHUB_RUN_ID': '123', 'GITHUB_ACTOR': 'synthetic-actor',
       'GITHUB_WORKFLOW_REF': probe.REPOSITORY + '/.github/workflows/release.yml@refs/heads/credential-probe-0807-abcdef012345',
       'GHCR_PROBE_TOKEN': SECRET, 'HOMEBREW_TAP_TOKEN': SECRET}


class Probe(unittest.TestCase):
    def run_main(self, channel, request):
        with tempfile.TemporaryDirectory() as temporary, patch.dict(os.environ, ENV, clear=True), \
                patch.object(probe, 'request', side_effect=request), contextlib.redirect_stdout(io.StringIO()) as output:
            previous = Path.cwd()
            try:
                os.chdir(temporary)
                code = probe.main([channel])
                report = json.loads(Path('credential-probe-' + channel + '.json').read_text())
            finally:
                os.chdir(previous)
            self.assertNotIn(SECRET, output.getvalue())
            self.assertNotIn('opaque-upload', output.getvalue())
            self.assertNotIn('opaque%2Fvalue', output.getvalue())
            return code, report

    def replies(self, status=202, location=LOCATION, cancel=204):
        calls = []
        def request(method, url, auth, parse=False):
            calls.append((method, url))
            if url == probe.TOKEN:
                return 200, {}, {'token': SECRET}
            if method == 'POST':
                return status, {'Location': location}, None
            if isinstance(cancel, Exception):
                raise cancel
            return cancel, {}, None
        return calls, request

    def test_empty_admission_is_immediately_cancelled_with_opaque_location_preserved(self):
        calls, request = self.replies()
        code, report = self.run_main('ghcr', request)
        self.assertEqual(code, 0)
        self.assertEqual(report['cleanup'], 'confirmed')
        self.assertEqual(calls, [('GET', probe.TOKEN), ('POST', probe.UPLOAD), ('DELETE', LOCATION)])
        self.assertIn('not_manifest', report['limit'])

    def test_path_relative_location_resolves_against_the_initiating_request(self):
        calls, request = self.replies(location='opaque-upload?state=opaque%2Fvalue&other=kept')
        code, report = self.run_main('ghcr', request)
        self.assertEqual(code, 0)
        self.assertEqual(calls[-1], ('DELETE', LOCATION))
        self.assertEqual(report['cleanup'], 'confirmed')

    def test_cleanup_attempted_on_unexpected_initiation_status(self):
        calls, request = self.replies(status=503)
        code, report = self.run_main('ghcr', request)
        self.assertEqual(code, 1)
        self.assertEqual(report['cleanup'], 'confirmed')
        self.assertEqual(calls[-1], ('DELETE', LOCATION))

    def test_failed_or_unknown_cleanup_cannot_pass(self):
        for status, expected in [(403, 'failed'), (404, 'already_absent'), (OSError(SECRET), 'failed')]:
            with self.subTest(status=type(status).__name__):
                _, request = self.replies(cancel=status)
                code, report = self.run_main('ghcr', request)
                self.assertEqual(code, 1)
                self.assertEqual(report['cleanup'], expected)

    def test_untrusted_locations_never_receive_credentials_or_delete(self):
        for value in ['https://other.invalid/v2/genomoncology/biomcp/blobs/uploads/id',
                      'https://ghcr.io:443/v2/genomoncology/biomcp/blobs/uploads/id',
                      'https://user@ghcr.io/v2/genomoncology/biomcp/blobs/uploads/id',
                      probe.UPLOAD + '../other', probe.UPLOAD + '%2e%2e/other',
                      probe.UPLOAD + '%252e%252e/other', probe.UPLOAD + 'id#fragment',
                      'http://ghcr.io/v2/genomoncology/biomcp/blobs/uploads/id',
                      'https://ghcr.io/v2/other/repo/blobs/uploads/id',
                      'https://ghcr.io/v2/genomoncology/biomcp/blobs/sha256:abc']:
            with self.subTest(location=value):
                calls, request = self.replies(location=value)
                code, report = self.run_main('ghcr', request)
                self.assertEqual(code, 1)
                self.assertEqual(report['error'], 'untrusted_upload_location')
                self.assertFalse(any(method == 'DELETE' for method, _ in calls))

    def test_authentication_denial_and_transport_errors_are_status_only(self):
        for outcome in [(403, {}, None), OSError(SECRET)]:
            def request(*_args, **_kwargs):
                if isinstance(outcome, Exception):
                    raise outcome
                return outcome
            code, report = self.run_main('ghcr', request)
            self.assertEqual(code, 1)
            self.assertNotIn(SECRET, json.dumps(report))

    def test_workflow_identity_rejects_other_repos_events_refs_before_requests(self):
        for key, value in [('GITHUB_REPOSITORY', 'example/scratch'), ('GITHUB_EVENT_NAME', 'push'),
                           ('GITHUB_REF', 'refs/heads/main'), ('GITHUB_WORKFLOW_REF', 'other')]:
            with patch.dict(os.environ, dict(ENV, **{key: value}), clear=True), \
                    self.assertRaisesRegex(probe.Refused, 'workflow_identity'):
                probe.context(os.environ)

    def test_http_boundary_rejects_redirects_and_never_sends_payloads(self):
        response = unittest.mock.MagicMock()
        response.__enter__.return_value = response
        response.code, response.headers = 202, {'Location': LOCATION}
        opener = unittest.mock.Mock()
        opener.open.return_value = response
        with patch.object(probe.urllib.request, 'build_opener', return_value=opener):
            probe.request('POST', probe.UPLOAD, 'Bearer ' + SECRET)
        req = opener.open.call_args.args[0]
        self.assertEqual(req.data, b'')
        self.assertEqual(req.full_url, probe.UPLOAD)
        response.read.assert_not_called()
        self.assertIsNone(probe.NoRedirect().redirect_request(None, None, 307, None, None, 'https://other.invalid'))
        for method, url in [('PUT', probe.UPLOAD), ('PATCH', probe.UPLOAD), ('POST', probe.UPLOAD + '?digest=x')]:
            with self.assertRaisesRegex(probe.Refused, 'request_scope'):
                probe.request(method, url, SECRET)

    def test_git_uses_only_dry_run_and_checks_remote_refs_without_credentials_in_argv(self):
        calls = []
        def git(args, root, env, data=None):
            calls.append(args)
            self.assertNotIn(SECRET, json.dumps(args))
            self.assertNotIn('GIT_TRACE', env)
            self.assertEqual(env['GIT_TERMINAL_PROMPT'], '0')
            if args[0] == 'rev-parse':
                return 'a' * 40
            if 'commit-tree' in args:
                self.assertEqual(args[-2:], ['-p', 'a' * 40])
                return 'b' * 40
            if args[0] == 'ls-remote':
                return 'a' * 40 + '\trefs/heads/main\n'
            return ''
        with patch.object(probe, 'git', side_effect=git):
            code, report = self.run_main('homebrew', lambda *_a, **_kw: (200, {}, {'full_name': 'genomoncology/homebrew-biomcp'}))
        self.assertEqual(code, 0)
        self.assertTrue(report['remote_refs_unchanged'])
        pushes = [args for args in calls if args[0] == 'push']
        self.assertEqual(pushes, [['push', '--dry-run', '--porcelain', probe.TAP, 'b' * 40 + ':refs/heads/main']])
        self.assertEqual(sum(args[0] == 'ls-remote' for args in calls), 2)

    def test_git_subprocess_captures_untrusted_output(self):
        with patch.object(probe.subprocess, 'run', return_value=subprocess.CompletedProcess([], 1, SECRET.encode(), SECRET.encode())):
            with self.assertRaisesRegex(probe.Refused, '^git_command_failed$'):
                probe.git(['push', '--dry-run'], Path('.'), {})

    def test_embedded_workflow_is_exact_and_only_manual_fixed_scope(self):
        text = Path('.github/workflows/channel-credentials.yml').read_text()
        embedded = text.split("<<'REVIEWED_PYTHON'\n", 1)[1].split('          REVIEWED_PYTHON\n', 1)[0]
        self.assertEqual(''.join(line[10:] + '\n' for line in embedded.splitlines()), Path('release/channel_credentials.py').read_text())
        self.assertIn('on:\n  workflow_dispatch:\n', text)
        for forbidden in ['push:', 'pull_request:', 'workflow_call:', 'environment:', 'actions/checkout', 'id-token:', 'contents: write', 'publication.py']:
            self.assertNotIn(forbidden, text)
        self.assertIn('packages: write', text)
        self.assertIn("github.repository == 'genomoncology/biomcp'", text)
        self.assertIn('GHCR_PROBE_TOKEN: ${{ github.token }}', text)
        self.assertIn('HOMEBREW_TAP_TOKEN: ${{ secrets.HOMEBREW_TAP_TOKEN }}', text)
        self.assertNotIn('env:\n  HOMEBREW', text)
        self.assertEqual(text.count('continue-on-error: true'), 2)
        self.assertIn('if: always()', text)
        self.assertIn('credential-probe-homebrew.json\n            credential-probe-ghcr.json', text)
        self.assertNotIn('path: .', text)


if __name__ == '__main__':
    unittest.main()

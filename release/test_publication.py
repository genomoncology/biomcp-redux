"""Publication contracts keep channel credentials behind approval and check installs."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


class Publication(unittest.TestCase):
    def test_every_channel_waits_for_approval_and_dispatch_cannot_publish(self):
        workflow = Path('.github/workflows/release.yml').read_text()
        for name in ['github', 'ghcr', 'pypi', 'homebrew', 'installer', 'registry']:
            job = workflow.split('\n  publish-' + name + ':\n', 1)[1].split('\n  ', 1)[0] if False else workflow.split('\n  publish-' + name + ':\n', 1)[1]
            import re
            job = re.split(r'\n  [a-z-]+:\n', job, maxsplit=1)[0]
            self.assertIn('environment: release-publication', job)
            for step in job.split('      - name: ')[1:]:
                if 'Report the approval rehearsal boundary' not in step:
                    self.assertIn("if: github.event_name == 'push' && github.repository == 'genomoncology/biomcp'", step)
        self.assertIn('attestations: true', workflow)
        self.assertIn('subject-digest: ${{ steps.image.outputs.digest }}', workflow)
        self.assertIn('push-to-registry: true', workflow)
        self.assertIn('make fmt lint test docs-check', workflow)
        post = workflow.split('\n  post-publish:\n')[1]
        for channel in ['pypi', 'ghcr', 'homebrew', 'installer', 'registry']:
            self.assertIn('channel: ' + channel, post)
            self.assertIn('publish-' + channel, post)

    def test_publication_command_refuses_unapproved_repository_event_or_source(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / 'candidate.json').write_text(json.dumps({'version': '1.0.0', 'revision': 'a' * 40}))
            base = {**os.environ, 'GITHUB_REPOSITORY': 'genomoncology/biomcp',
                    'GITHUB_EVENT_NAME': 'push', 'GITHUB_REF': 'refs/tags/v1.0.0', 'GITHUB_SHA': 'a' * 40}
            for name, value in [('GITHUB_REPOSITORY', 'example/scratch'), ('GITHUB_EVENT_NAME', 'workflow_dispatch'),
                                ('GITHUB_REF', 'refs/heads/main'), ('GITHUB_SHA', 'b' * 40)]:
                result = subprocess.run([sys.executable, 'release/publication.py', 'wheels', '--staged', str(root)],
                                        env={**base, name: value}, capture_output=True, text=True)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn('publication identity', result.stderr)

    def test_version_checker_rejects_successful_wrong_version(self):
        from release.published import check_version
        for output in ['biomcp 0.9.3\n', 'biomcp 1.0.0\nextra\n', '']:
            with self.assertRaises(ValueError):
                check_version(output, '1.0.0')
        check_version('biomcp 1.0.0\n', '1.0.0')


if __name__ == '__main__':
    unittest.main()

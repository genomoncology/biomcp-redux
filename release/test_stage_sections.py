"""Exercise recovery artifact consumption at its file and API boundaries."""
import copy
import json
import os
import subprocess
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import urllib.request
import zipfile

from release import stage_sections as consumer


class RecoveryConsumption(unittest.TestCase):
    def fixture(self, root, platform='macosx_11_0_arm64'):
        dist = root / 'workspace/product/dist'
        dist.mkdir(parents=True)
        record = {'revision': consumer.PRODUCT, 'platform': platform,
                  'distribution': 'biomcp-cli', 'version': '1.0.0'}
        (dist / 'release.json').write_text(json.dumps(record))
        kinds = ['wheel', 'desktop-extension', 'mcp-registry', 'install-script',
                 'download-install-script', 'homebrew-formula']
        report = [{'artifact': kind, 'platform': platform, 'result': 'pass',
                   'cases': 611, 'trials': 611, 'checks': ['candidate-bytes',
                   'concurrent-first-opens', 'all-cases-and-trials', 'mcp-all-stdout']}
                  for kind in kinds]
        (dist / 'install-report.json').write_text(json.dumps(report))
        evidence = root / 'workspace/section-evidence'
        evidence.mkdir()
        identities = {'product': {'revision': consumer.PRODUCT, 'tree': consumer.PRODUCT_TREE},
                      'tooling': {'revision': consumer.RECOVERY, 'tree': consumer.RECOVERY_TREE}}
        (evidence / 'sources.json').write_text(json.dumps({**identities,
            'publishing': False, 'original_run': '42030695983'}))
        receipt = {**identities, 'run': consumer.RUNS[platform], 'section': platform,
                   'sha256': {'dist/' + path.name: consumer.digest(path) for path in dist.iterdir()}}
        (evidence / 'artifacts.json').write_text(json.dumps(receipt))
        return dist, evidence

    def test_nested_final_recovery_consumes_only_bound_file_inventory(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            dist, _ = self.fixture(root)
            selected, evidence = consumer.validate_payload(root, 'macosx_11_0_arm64',
                                                           consumer.RUNS['macosx_11_0_arm64'])
            self.assertEqual(selected, dist)
            (dist / 'fixture-evidence').mkdir()
            consumer.validate_payload(root, 'macosx_11_0_arm64', consumer.RUNS['macosx_11_0_arm64'])
            (dist / 'fixture-evidence/unlisted').write_bytes(b'not an empty evidence directory')
            with self.assertRaisesRegex(ValueError, 'inventory'):
                consumer.validate_payload(root, 'macosx_11_0_arm64', consumer.RUNS['macosx_11_0_arm64'])
            (dist / 'fixture-evidence/unlisted').unlink()
            self.assertEqual(evidence['sources']['product']['revision'], consumer.PRODUCT)
            (dist / 'unlisted.whl').write_bytes(b'foreign bytes')
            with self.assertRaisesRegex(ValueError, 'inventory'):
                consumer.validate_payload(root, 'macosx_11_0_arm64', consumer.RUNS['macosx_11_0_arm64'])

    def test_source_platform_and_incomplete_qualification_are_rejected(self):
        mutations = [('sources.json', lambda x: x['product'].update(revision='f' * 40)),
                     ('artifacts.json', lambda x: x.update(section='win_amd64')),
                     ('artifacts.json', lambda x: x['sha256'].update({'dist/release.json': '0' * 64}))]
        for name, mutate in mutations:
            with self.subTest(name=name), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                _, evidence = self.fixture(root)
                path = evidence / name
                record = json.loads(path.read_text()); mutate(record)
                path.write_text(json.dumps(record))
                with self.assertRaises(ValueError):
                    consumer.validate_payload(root, 'macosx_11_0_arm64', consumer.RUNS['macosx_11_0_arm64'])
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            dist, evidence = self.fixture(root)
            report = json.loads((dist / 'install-report.json').read_text())[:-1]
            (dist / 'install-report.json').write_text(json.dumps(report))
            record = json.loads((evidence / 'artifacts.json').read_text())
            record['sha256']['dist/install-report.json'] = consumer.digest(dist / 'install-report.json')
            (evidence / 'artifacts.json').write_text(json.dumps(record))
            with self.assertRaisesRegex(ValueError, 'channel inventory'):
                consumer.validate_payload(root, 'macosx_11_0_arm64', consumer.RUNS['macosx_11_0_arm64'])

    def test_recovery_api_requires_exact_successful_run_source(self):
        run = {'id': int(consumer.RUNS['win_amd64']), 'head_sha': consumer.RECOVERY,
               'event': 'workflow_dispatch', 'path': '.github/workflows/release.yml',
               'repository': {'full_name': consumer.REPOSITORY},
               'head_repository': {'full_name': consumer.REPOSITORY},
               'status': 'completed', 'conclusion': 'success', 'run_attempt': 1,
               'head_branch': 'rehearsal-0642-reviewed'}
        consumer.validate_run(run, 'win_amd64')
        for key, value in [('id', 1), ('head_sha', consumer.PRODUCT), ('conclusion', 'failure'),
                           ('event', 'push'), ('run_attempt', 2), ('head_branch', 'main')]:
            altered = copy.deepcopy(run); altered[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                consumer.validate_run(altered, 'win_amd64')

    def test_unsafe_zip_and_cross_host_token_forwarding_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with zipfile.ZipFile(root / 'bad.zip', 'w') as archive:
                archive.writestr('../outside', b'bad')
            with self.assertRaises(ValueError):
                consumer.unpack(root / 'bad.zip', root / 'output')
            self.assertFalse((root / 'outside').exists())
        request = urllib.request.Request('https://api.github.com/repos/example',
                                         headers={'Authorization': 'Bearer fixture'})
        redirected = consumer.ObjectRedirect().redirect_request(request, None, 302, '', {},
                                                                'https://objects.example/archive')
        self.assertIsNone(redirected.get_header('Authorization'))

    def test_source_guard_rejects_packaging_changes_beyond_product_rust(self):
        # Real independent Git checkouts exercise the source boundary, including normalization.
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            def git(path, *args):
                return subprocess.check_output(['git', '-C', str(path), *args], text=True,
                                               stderr=subprocess.DEVNULL).strip()
            for name, revision in [('product', consumer.PRODUCT), ('tooling', 'HEAD')]:
                subprocess.run(['git', 'clone', '--shared', '--no-checkout', '.', str(root / name)],
                               check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                git(root / name, 'checkout', '--detach', revision)
            tooling = root / 'tooling'
            workflow = tooling / '.github/workflows/release-stage-sections.yml'
            (tooling / '.github/workflows/release.yml').write_bytes(workflow.read_bytes())
            def commit():
                git(tooling, 'add', '.')
                git(tooling, '-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                    'commit', '-m', 'Normalize fixture deployment')
                return git(tooling, 'rev-parse', 'HEAD')
            with patch.dict(os.environ, {'GITHUB_SHA': commit(),
                    'GITHUB_REF': 'refs/heads/rehearsal-0370-fixture',
                    'GITHUB_WORKFLOW_REF': 'fixture/.github/workflows/release.yml@refs/heads/rehearsal-0370-fixture',
                    'GITHUB_RUN_ID': '1'}):
                identity = consumer.source_guard(root / 'product', tooling)
                self.assertEqual(identity['product_revision'], consumer.PRODUCT)
                self.assertNotEqual(identity['consumer_revision'], consumer.PRODUCT)
                (tooling / 'release/install.sh').write_text('# changed packaging input\n')
                os.environ['GITHUB_SHA'] = commit()
                with self.assertRaisesRegex(ValueError, 'packaging input'):
                    consumer.source_guard(root / 'product', tooling)

    def test_same_sanitized_recovery_receipt_becomes_a_signed_subject(self):
        import hashlib
        import yaml
        run = dict(id=1, head_sha=consumer.RECOVERY, event='workflow_dispatch',
                   path='.github/workflows/release.yml', head_branch='reviewed',
                   status='completed', conclusion='success', run_attempt=1,
                   arbitrary_url='https://object.invalid/?token=do-not-retain')
        record = consumer.api_receipt(run, [])
        self.assertNotIn('do-not-retain', json.dumps(record))
        workflow = yaml.safe_load(Path('.github/workflows/release-stage-sections.yml').read_text())
        steps = workflow['jobs']['stage']['steps']
        copy_index, = [i for i, step in enumerate(steps)
                       if step.get('name') == 'Bind the rehearsal inputs to the signed staged payload']
        scan_index, = [i for i, step in enumerate(steps) if 'secrets.py artifacts' in step.get('run', '')]
        subject_index, = [i for i, step in enumerate(steps) if 'workflow.py subjects' in step.get('run', '')]
        attest_index, = [i for i, step in enumerate(steps) if step.get('uses', '').startswith('actions/attest@')]
        self.assertLess(copy_index, scan_index)
        self.assertLess(scan_index, subject_index)
        self.assertLess(subject_index, attest_index)
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'receipts').mkdir(); (root / 'staged').mkdir()
            receipt = json.dumps(record).encode()
            (root / 'receipts/recovery-inputs.json').write_bytes(receipt)
            subprocess.run(['sh', '-ec', steps[copy_index]['run']], cwd=root, check=True)
            subprocess.run([os.sys.executable, str(Path('release/workflow.py').resolve()), 'subjects',
                            '--artifacts', str(root / 'staged'), '--output', str(root / 'subjects.sha256')],
                           check=True)
            self.assertEqual((root / 'receipts/recovery-inputs.json').read_bytes(), receipt)
            self.assertEqual((root / 'staged/REHEARSAL-INPUT-BINDING.json').read_bytes(), receipt)
            self.assertEqual((root / 'subjects.sha256').read_text(),
                             hashlib.sha256(receipt).hexdigest() + '  REHEARSAL-INPUT-BINDING.json\n')

    def test_consumer_workflow_has_only_stage_signing_and_no_publication(self):
        # Keep this boundary check structural; the artifact cases above own aggregation behavior.
        import yaml
        workflow = yaml.safe_load(Path('.github/workflows/release-stage-sections.yml').read_text())
        self.assertEqual(set(workflow['on']), {'workflow_dispatch'})
        jobs = workflow['jobs']
        self.assertNotIn('if', jobs['stage'])  # Invalid inputs fail the helper rather than skipping green.
        for name, job in jobs.items():
            self.assertNotIn('environment', job)
            writes = {key for key, value in job['permissions'].items() if value == 'write'}
            self.assertEqual(writes, {'id-token', 'attestations'} if name == 'stage' else set())
            for step in job['steps']:
                self.assertNotIn('${{ secrets.', json.dumps(step))
                self.assertNotIn('publication.py', step.get('run', ''))
                self.assertNotIn('published.py', step.get('run', ''))
        attest, = [step for step in jobs['stage']['steps'] if step.get('uses', '').startswith('actions/attest@')]
        self.assertNotIn('if', attest)
        self.assertFalse(attest['with']['push-to-registry'])
        brew = jobs['staged-homebrew']
        self.assertEqual(len(brew['strategy']['matrix']['include']), 2)
        gate, = [step for step in brew['steps'] if 'release/gate.py' in step.get('run', '')]
        self.assertIn('$GITHUB_WORKSPACE/formula', gate['run'])
        self.assertEqual(gate['env']['BIOMCP_BUILD_REVISION'], '${{ env.PRODUCT_REVISION }}')
        self.assertNotIn('sandbox', gate['run'])


if __name__ == '__main__':
    unittest.main()

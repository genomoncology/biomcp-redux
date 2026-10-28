"""Release contracts at the tool check and cold rehearsal command boundaries."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class ReleaseTools(unittest.TestCase):
    def test_toolchain_drift_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            shutil.copytree(ROOT / 'release', root / 'release')
            shutil.copytree(ROOT / '.github', root / '.github')
            shutil.copyfile(ROOT / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            command = [sys.executable, str(root / 'release/toolchain.py'), 'check']
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            mutations = {
                'rust-toolchain.toml': lambda s: s.replace('1.95.0', '1.94.0'),
                'release/linux.sh': lambda s: s + '\n# rustup/archive/0.0.1\n',
                'release/matrix.sh': lambda s: s + '\n# --default-toolchain 1.94.0\n',
                'release/secrets.py': lambda s: s + '\nVERSION = "0.0.1"\n',
                '.github/workflows/release.yml': lambda s: s.replace(
                    'python-version: ${{ steps.tools.outputs.python }}', "python-version: '3.11'"),
            }
            for name, mutate in mutations.items():
                with self.subTest(recipe=name):
                    path = root / name
                    original = path.read_text()
                    path.write_text(mutate(original))
                    result = subprocess.run(command, capture_output=True, text=True)
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn(Path(name).name, result.stderr)
                    path.write_text(original)

    def test_cold_stages_report_all_failures_and_run_later_stages(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'release').mkdir()
            shutil.copyfile(ROOT / 'release/rehearse.py', root / 'release/rehearse.py')
            shutil.copyfile(ROOT / 'release/toolchain.json', root / 'release/toolchain.json')
            stub = '''import os, sys\nfrom pathlib import Path\nwith open('seen', 'a') as stream: stream.write(Path(sys.argv[0]).stem + ':' + ' '.join(sys.argv[1:]) + '\\n')\nsys.exit(7 if Path(sys.argv[0]).stem == 'artifacts' or (Path(sys.argv[0]).stem == 'toolchain' and sys.argv[1] == 'install-rust') else 0)\n'''
            for name in ('toolchain', 'artifacts', 'gate', 'workflow', 'secrets'):
                (root / ('release/' + name + '.py')).write_text(stub)
            bindir = root / 'bin'
            bindir.mkdir()
            for name in ('cargo', 'make'):
                executable = bindir / name
                executable.write_text('#!' + sys.executable + '\n' + stub)
                executable.chmod(0o755)
            env = dict(os.environ, PATH=str(bindir) + os.pathsep + os.environ['PATH'],
                       BIOMCP_BUILD_REVISION='a' * 40)
            report = root / 'report.json'
            result = subprocess.run([sys.executable, 'release/rehearse.py', 'inside',
                                     '--report', str(report)], cwd=root, env=env,
                                    capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            rows = json.loads(report.read_text())
            self.assertEqual([row['stage'] for row in rows if row['exit'] != 0],
                             ['toolchain', 'build'])
            self.assertEqual(rows[-1]['stage'], 'schema')
            self.assertEqual(rows[-1]['exit'], 0)
            self.assertIn('gate:--prepare-only', (root / 'seen').read_text())
            self.assertIn('secrets:history', (root / 'seen').read_text())
            print('Seeded failures: toolchain=7, build=7; later prepare and schema ran')

    @unittest.skipIf(os.name == 'nt', 'the Linux recipe requires a POSIX host')
    def test_cold_container_failures_do_not_stop_install_or_container_stages(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            shutil.copytree(ROOT / 'release', root / 'release')
            bindir = root / 'bin'
            bindir.mkdir()
            docker = bindir / 'docker'
            docker.write_text('#!' + sys.executable + '\nimport json,sys\n'
                              'with open("docker.jsonl", "a") as stream: stream.write(json.dumps(sys.argv[1:]) + "\\n")\n'
                              'sys.exit(23)\n')
            docker.chmod(0o755)
            report = root / 'report.json'
            env = dict(os.environ, PATH=str(bindir) + os.pathsep + os.environ['PATH'],
                       BUILD_IMAGE='cold-image', BIOMCP_RELEASE_COLD='1',
                       BIOMCP_BUILD_REVISION='a' * 40, BIOMCP_RELEASE_REPORT=str(report))
            result = subprocess.run(['sh', 'release/linux.sh', '--label', 'example.owner=cold'],
                                    cwd=root, env=env, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            rows = json.loads(report.read_text())
            self.assertEqual([row['stage'] for row in rows if row['exit'] == 23],
                             ['build', 'install', 'container-build', 'container-save', 'container-cleanup'])
            calls = [json.loads(line) for line in (root / 'docker.jsonl').read_text().splitlines()]
            for call in calls[:2]:
                self.assertIn('--rm', call)
                self.assertNotIn(':/cargo', ' '.join(call))
                self.assertIn('example.owner=cold', call)
            self.assertIn('CARGO_HOME=/tmp/release-cargo', calls[0][-1])
            self.assertFalse((root / '.release-cache').exists())


if __name__ == '__main__':
    unittest.main()

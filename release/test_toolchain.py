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
            shutil.copyfile(ROOT / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            result = subprocess.run([sys.executable, str(root / 'release/toolchain.py'), 'check'],
                                    capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            manifest = root / 'rust-toolchain.toml'
            manifest.write_text(manifest.read_text().replace('1.95.0', '1.94.0'))
            result = subprocess.run([sys.executable, str(root / 'release/toolchain.py'), 'check'],
                                    capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('rust-toolchain.toml', result.stderr)

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


if __name__ == '__main__':
    unittest.main()

"""Local release commands refuse to mislabel modified source as a clean commit."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class SourceRevision(unittest.TestCase):
    def test_local_recipes_refuse_dirty_source_before_build_or_download(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / 'source'
            subprocess.run(['git', 'clone', '--quiet', '--shared', str(ROOT), str(root)], check=True)
            (root / 'README.md').write_text('modified source\n')
            bindir = Path(directory) / 'bin'
            bindir.mkdir()
            for name in ('cargo', 'docker'):
                command = bindir / name
                command.write_text('#!/bin/sh\necho unexpected-build >&2\nexit 91\n')
                command.chmod(0o755)
            env = dict(os.environ, PATH=str(bindir) + os.pathsep + os.environ['PATH'], BUILD_IMAGE='fixture')
            env.pop('BIOMCP_BUILD_REVISION', None)
            for command in ([sys.executable, 'release/artifacts.py', 'build', '--output', 'dist'],
                            ['sh', 'release/linux.sh'], ['sh', 'release/matrix.sh'],
                            [sys.executable, 'release/rehearse.py', 'cold', '--report', 'dist/report.json']):
                with self.subTest(command=command):
                    result = subprocess.run(command, cwd=root, env=env, capture_output=True, text=True, timeout=30)
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn('dirty source tree', result.stderr)
                    self.assertNotIn('unexpected-build', result.stderr)
            command = [sys.executable, str(root / 'release/source_revision.py'), '--revision', 'a' * 40]
            result = subprocess.run(command, cwd=root, env=env, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(result.stdout.strip(), 'a' * 40)
            result = subprocess.run(command[:-2], cwd=root, env=dict(env, BIOMCP_BUILD_REVISION='b' * 40),
                                    capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(result.stdout.strip(), 'b' * 40)


if __name__ == '__main__':
    unittest.main()

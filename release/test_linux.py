"""Exercise Linux release container arguments through the Docker boundary."""
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile
import unittest

from release.smoke import environment

ROOT = Path(__file__).resolve().parents[1]
DOCKER = r'''import hashlib, io, json, os, pathlib, sys, tarfile
args = sys.argv[1:]
with open(os.environ['DOCKER_RECORD'], 'a') as stream:
    stream.write(json.dumps(args) + '\n')
if args[0] != 'run':
    sys.exit(0)
if '/bin/bash' in args:
    if '/artifacts:ro' not in ' '.join(args):
        pathlib.Path('dist').mkdir()
        pathlib.Path('dist/release.json').write_text('{"version":"1.0.0"}')
        with tarfile.open('dist/biomcp-linux-x86_64.tar.gz', 'w:gz') as archive:
            info = tarfile.TarInfo('biomcp')
            info.size = 9
            archive.addfile(info, io.BytesIO(b'candidate'))
        pathlib.Path('proof/fixtures').mkdir(parents=True)
        pathlib.Path('proof/case-runner').touch()
    else:
        print('[]')
elif '/usr/bin/sha256sum' in args:
    print(hashlib.sha256(b'candidate').hexdigest(), '/usr/local/bin/biomcp')
elif args[-2:] == ['schema', 'biodata.problem/1']:
    print('{"$id":"biodata.problem/1"}')
elif args[-1] == 'help':
    print('{"result":{"commands":["get"]}}')
elif args[-1] == 'serve':
    for frame in sys.stdin:
        value = json.loads(frame)
        if 'id' not in value:
            continue
        result = ({'serverInfo': {'version': '1.0.0'}} if value['id'] == 1 else
                  {'tools': [{'name': 'biomcp'}]} if value['id'] == 2 else {})
        print(json.dumps({'jsonrpc': '2.0', 'id': value['id'], 'result': result}))
'''


@unittest.skipIf(sys.platform == 'win32', 'the release script requires a POSIX shell')
class LinuxContainers(unittest.TestCase):
    def run_release(self, extra):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'release').mkdir()
            for name in ('linux.sh', 'container_check.py', 'smoke.py', 'Dockerfile'):
                shutil.copyfile(ROOT / 'release' / name, root / 'release' / name)
            shutil.copyfile(ROOT / 'Makefile', root / 'Makefile')
            bindir = root / 'bin'
            bindir.mkdir()
            docker = bindir / 'docker'
            docker.write_text('#!' + sys.executable + '\n' + DOCKER)
            docker.chmod(0o755)
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            subprocess.run(['git', '-C', str(root), '-c', 'user.name=Fixture',
                            '-c', 'user.email=fixture@example.org', 'commit',
                            '--allow-empty', '-qm', 'Initialize fixture'], check=True)
            record = root / 'docker.jsonl'
            env = environment(root)
            env.update(PATH=str(bindir) + os.pathsep + os.environ['PATH'],
                       BUILD_IMAGE='build-image', DOCKER_RECORD=str(record))
            result = subprocess.run(['make', 'release-linux', 'CONTAINER_ARGS=' + shlex.join(extra)],
                                    cwd=root, env=env, capture_output=True, text=True, timeout=30)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertFalse((root / 'injected').exists())
            return [json.loads(line) for line in record.read_text().splitlines()]

    def test_every_started_container_receives_only_the_callers_extra_arguments(self):
        extra = ['--label', 'example.owner=release check', '--label',
                 'example.literal=$(touch injected);*', '--network', 'none']
        baseline = self.run_release([])
        supplied = self.run_release(extra)
        self.assertEqual(len(baseline), 12)
        self.assertEqual(sum(row[0] == 'run' for row in baseline), 9)
        for plain, labeled in zip(baseline, supplied):
            with self.subTest(command=plain):
                self.assertNotIn('--label', plain)
                if plain[0] == 'run':
                    start = labeled.index(extra[0])
                    self.assertEqual(labeled[start:start + len(extra)], extra)
                    labeled = labeled[:start] + labeled[start + len(extra):]
                # Temporary source paths and the checkout revision vary per run.
                normalize = lambda row: [arg.replace(arg.split(':/')[0], '<mount>')
                                        if arg.startswith('/') and ':/' in arg else
                                        '<revision>' if arg.startswith('BIOMCP_BUILD_REVISION=') else arg
                                        for arg in row]
                self.assertEqual(normalize(labeled), normalize(plain))
        self.assertEqual([row[-1] for row in baseline[3:10]],
                         ['/usr/local/bin/biomcp', 'biodata.problem/1', 'help', 'serve',
                          '1', '--installed-process-proof', '--installed-http-proof'])


if __name__ == '__main__':
    unittest.main()

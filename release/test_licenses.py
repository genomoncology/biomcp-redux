"""Release contract: callers reject artifacts that lose required notices."""
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import unittest
import zipfile

LICENSE = Path('LICENSE').read_bytes()
NOTICES = b'Generated third-party notices\nSynthetic fixture dependency\nPermission notice\n'


def payloads():
    return {'LICENSE': LICENSE, 'THIRD-PARTY-NOTICES.txt': NOTICES}


def write_payloads(directory):
    for name, data in payloads().items():
        (directory / name).write_bytes(data)


def archive(path, files):
    if path.name.endswith(('.tar.gz', '.tar')):
        with tarfile.open(path, 'w:gz' if path.name.endswith('.gz') else 'w') as target:
            for name, value in files.items():
                entry = tarfile.TarInfo(name)
                entry.size = len(value)
                target.addfile(entry, io.BytesIO(value))
    else:
        with zipfile.ZipFile(path, 'w') as target:
            for name, value in files.items():
                target.writestr(name, value)


def wheel_fixture(directory, tag):
    binary = tag.encode()
    record = {'version': '1.0.0', 'revision': 'a' * 40, 'platform': tag,
              'distribution': 'biomcp-cli', 'profile': 'release',
              'cargo_profile': {'debug_assertions': False, 'opt_level': '3'},
              'binary_sha256': hashlib.sha256(binary).hexdigest()}
    prefix = 'biomcp_cli-1.0.0.dist-info/'
    suffix = '.exe' if tag.startswith('win_') else ''
    files = {prefix + name: value for name, value in payloads().items()}
    files.update({prefix + 'release.json': json.dumps(record).encode(),
                  prefix + 'METADATA': b'Name: biomcp-cli\nVersion: 1.0.0\nLicense: MIT\nProject-URL: Homepage, https://biomcp.org\n\nmcp-name: io.github.genomoncology/biomcp\n',
                  prefix + 'WHEEL': ('Tag: py3-none-' + tag + '\n').encode(),
                  'biomcp_cli-1.0.0.data/scripts/biomcp' + suffix: binary,
                  'biomcp_cli-1.0.0.data/scripts/biomcp-cli' + suffix: binary})
    path = directory / ('biomcp_cli-1.0.0-py3-none-' + tag + '.whl')
    archive(path, files)
    return path, record


class Licensing(unittest.TestCase):
    def test_inspection_requires_license_notices_and_registry_metadata(self):
        for defect in ('LICENSE', 'THIRD-PARTY-NOTICES.txt', 'marker', 'License', 'Project-URL', None):
            with self.subTest(defect=defect), tempfile.TemporaryDirectory() as temp:
                path, _ = wheel_fixture(Path(temp), 'linux_x86_64')
                with zipfile.ZipFile(path) as source:
                    files = {name: source.read(name) for name in source.namelist()}
                if defect in payloads():
                    del files['biomcp_cli-1.0.0.dist-info/' + defect]
                elif defect:
                    name = 'biomcp_cli-1.0.0.dist-info/METADATA'
                    token = {'marker': b'mcp-name: io.github.genomoncology/biomcp', 'License': b'License: MIT',
                             'Project-URL': b'Project-URL: Homepage, https://biomcp.org'}[defect]
                    files[name] = files[name].replace(token, b'')
                archive(path, files)
                result = subprocess.run([sys.executable, 'release/artifacts.py', 'inspect', str(path)], capture_output=True, text=True)
                self.assertEqual(result.returncode == 0, defect is None, result.stderr)

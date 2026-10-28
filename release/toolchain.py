"""Read the release tool pins and bootstrap the pinned Rust installer."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import sys
import tempfile
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


def manifest():
    return json.loads((ROOT / 'release/toolchain.json').read_text())


def check():
    pins = manifest()
    source = (ROOT / 'rust-toolchain.toml').read_text()
    if not re.search(r'^channel\s*=\s*"' + re.escape(pins['rust']) + r'"\s*$', source, re.M):
        raise ValueError('rust-toolchain.toml differs from release/toolchain.json')
    # Recipes consume these pins rather than maintaining independent versions.
    for name in ('linux.sh', 'matrix.sh'):
        text = (ROOT / 'release' / name).read_text()
        if 'toolchain.py' not in text or re.search(r'(?:--default-toolchain\s+\d|/cp\d|rustup/archive/\d)', text):
            raise ValueError(name + ' must consume the release tool manifest')
    scanner = (ROOT / 'release/secrets.py').read_text()
    if re.search(r'^VERSION\s*=\s*[\"\']', scanner, re.M):
        raise ValueError('secrets.py duplicates the scanner pin')
    workflow = ROOT / '.github/workflows/release.yml'
    if workflow.exists():
        text = workflow.read_text()
        for field in ('python-version', 'toolchain'):
            if re.search(r'^\s+' + field + r':\s*[\'\"]?\d', text, re.M):
                raise ValueError('release.yml duplicates a ' + field + ' pin')
        if re.search(r'cargo install cargo-deny --version\s+\d', text):
            raise ValueError('release.yml duplicates the cargo-deny pin')
    print('Release tool pins agree')


def install_rust():
    pins = manifest()
    system, machine = platform.system(), platform.machine().lower()
    arch = {'arm64': 'aarch64', 'amd64': 'x86_64'}.get(machine, machine)
    target = arch + {'Linux': '-unknown-linux-gnu', 'Darwin': '-apple-darwin',
                     'Windows': '-pc-windows-msvc'}[system]
    suffix = '.exe' if system == 'Windows' else ''
    base = 'https://static.rust-lang.org/rustup/archive/' + pins['rustup'] + '/' + target + '/rustup-init' + suffix
    with tempfile.TemporaryDirectory() as directory:
        installer = Path(directory) / ('rustup-init' + suffix)
        with urllib.request.urlopen(base, timeout=120) as response:
            payload = response.read()
        with urllib.request.urlopen(base + '.sha256', timeout=120) as response:
            expected = response.read().decode().split()[0]
        if hashlib.sha256(payload).hexdigest() != expected:
            raise ValueError('rustup installer checksum differs')
        installer.write_bytes(payload)
        installer.chmod(0o700)
        subprocess.run([str(installer), '-y', '--no-modify-path', '--profile', 'minimal',
                        '--default-toolchain', pins['rust']], check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=['get', 'outputs', 'check', 'install-rust', 'python-check'])
    parser.add_argument('key', nargs='?')
    args = parser.parse_args()
    pins = manifest()
    if args.command == 'get':
        print(pins[args.key])
    elif args.command == 'outputs':
        for key in ('rust', 'rustup', 'python', 'cargo-deny'):
            print(key.replace('-', '_') + '=' + pins[key])
    elif args.command == 'check':
        check()
    elif args.command == 'install-rust':
        install_rust()
    else:
        expected = tuple(map(int, pins['python'].split('.')))
        if sys.version_info[:len(expected)] != expected:
            raise ValueError('Python differs from release tool pin: ' + pins['python'])


if __name__ == '__main__':
    try:
        main()
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        sys.exit(str(error))

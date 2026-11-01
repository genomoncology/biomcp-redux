"""Publish the staged candidate from an approved production tag job."""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

REPOSITORY = 'genomoncology/biomcp'
IMAGE = 'ghcr.io/' + REPOSITORY


def run(command, **kwargs):
    return subprocess.check_output(command, text=True, **kwargs)


def identity(staged):
    record = json.loads((staged / 'candidate.json').read_text())
    version, revision = record['version'], record['revision']
    if (not re.fullmatch(r'\d+\.\d+\.\d+', version)
            or not re.fullmatch(r'[0-9a-f]{40}', revision)
            or os.environ.get('GITHUB_REPOSITORY') != REPOSITORY
            or os.environ.get('GITHUB_EVENT_NAME') != 'push'
            or os.environ.get('GITHUB_REF') != 'refs/tags/v' + version
            or os.environ.get('GITHUB_SHA') != revision):
        raise ValueError('publication identity must match the production tag and staged source')
    return version


def github(staged, version):
    # Upload only the staged public assets; the saved container images go to GHCR.
    assets = [str(path) for path in sorted(staged.iterdir())
              if path.name != 'RELEASE-NOTES.md' and not path.name.startswith('biomcp-container-')]
    run(['gh', 'release', 'create', 'v' + version, '--repo', REPOSITORY,
         '--verify-tag', '--title', 'BioMCP ' + version, '--notes-file', str(staged / 'RELEASE-NOTES.md'),
         '--latest=false', *assets])


def ghcr(staged, version):
    references = []
    for arch in ('amd64', 'arm64'):
        subprocess.run(['docker', 'load', '--input', str(staged / ('biomcp-container-' + arch + '.tar'))],
                       check=True, stdout=sys.stderr)
        reference = IMAGE + ':' + version + '-' + arch
        subprocess.run(['docker', 'tag', 'biomcp-candidate:' + arch, reference], check=True)
        subprocess.run(['docker', 'push', reference], check=True, stdout=sys.stderr)
        references.append(reference)
    tag = IMAGE + ':' + version
    subprocess.run(['docker', 'manifest', 'create', tag, *references], check=True, stdout=sys.stderr)
    output = run(['docker', 'manifest', 'push', tag])
    digest = output.strip().splitlines()[-1]
    if not re.fullmatch(r'sha256:[0-9a-f]{64}', digest):
        raise ValueError('registry did not return the pushed manifest digest')
    # The action attests this returned registry digest, never a saved image tar.
    print('digest=' + digest)


def wheels(staged, version):
    output = Path('pypi')
    output.mkdir(exist_ok=False)
    candidates = sorted(staged.glob('*.whl'))
    if len(candidates) != 5:
        raise ValueError('publication requires all five qualified wheels')
    for path in candidates:
        shutil.copy2(path, output / path.name)


def commit_payload(checkout, source, target, version, branch):
    target = checkout / target
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, target)
    run(['git', '-C', str(checkout), 'add', '--', str(target.relative_to(checkout))])
    if not run(['git', '-C', str(checkout), 'diff', '--cached', '--name-only']).strip():
        return
    run(['git', '-C', str(checkout), '-c', 'user.name=github-actions[bot]',
         '-c', 'user.email=41898282+github-actions[bot]@users.noreply.github.com',
         'commit', '-m', 'Publish BioMCP ' + version])
    run(['git', '-C', str(checkout), 'push', 'origin', 'HEAD:' + branch])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('channel', choices=['github', 'ghcr', 'wheels', 'homebrew', 'installer', 'registry'])
    parser.add_argument('--staged', type=Path, required=True)
    parser.add_argument('--checkout', type=Path)
    args = parser.parse_args()
    version = identity(args.staged)
    if args.channel in ('github', 'ghcr', 'wheels'):
        globals()[args.channel](args.staged, version)
    elif args.channel == 'registry':
        subprocess.run(['mcp-publisher', 'login', 'github-oidc'], cwd=args.staged, check=True)
        subprocess.run(['mcp-publisher', 'publish'], cwd=args.staged, check=True)
    else:
        if args.checkout is None:
            raise ValueError('publication requires the destination checkout')
        if args.channel == 'homebrew':
            commit_payload(args.checkout, args.staged / 'biomcp.rb', Path('Formula/biomcp.rb'), version, 'main')
        else:
            commit_payload(args.checkout, args.staged / 'install.sh', Path('install.sh'), version, 'gh-pages')
            run(['gh', 'api', '--method', 'POST', 'repos/' + REPOSITORY + '/pages/builds'])


if __name__ == '__main__':
    try:
        main()
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        sys.exit(str(error))

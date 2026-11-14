"""Run the library tests of every crate the branch changed since it left main.

The landing gate runs this so a test added or changed under a ticket runs before
the ticket lands (0914). It names the base, the changed crates and each cargo
command in the log, and cargo names every test binary and test it runs.
A changed crate also selects every crate that depends on it, read from
`cargo metadata` (0921). The shared test data under tests/fixtures/, tests/replies/
and tests/native/ counts as a change to biomcp-test-support, which serves it, so it
selects every crate whose tests use that helper (direct readers use it too).
A change to the workspace manifest, lock file or toolchain selects every crate.
When no base or crate graph can be found it runs every crate rather than none.
"""
import json
import os
import subprocess
import sys

BASE_REF = os.environ.get('LIBRARY_TESTS_BASE', 'origin/main')
WORKSPACE_FILES = ('Cargo.toml', 'Cargo.lock', 'rust-toolchain', 'rust-toolchain.toml')
WORKSPACE_DIRS = ('.cargo/',)
SHARED_TEST_DATA = ('tests/fixtures/', 'tests/replies/', 'tests/native/')
TEST_DATA_CRATE = 'biomcp-test-support'


def git(*args):
    return subprocess.run(['git', *args], check=True, capture_output=True, text=True).stdout.strip()


def crates():
    return sorted(name for name in os.listdir('crates') if os.path.isfile(os.path.join('crates', name, 'Cargo.toml')))


def graph():
    """Map each workspace crate to its workspace dependencies, split by kind."""
    command = ['cargo', 'metadata', '--no-deps', '--format-version', '1', '--locked']
    packages = json.loads(subprocess.run(command, check=True, capture_output=True, text=True).stdout)['packages']
    names = {package['name'] for package in packages}
    edges = {}
    for package in packages:
        normal, dev = set(), set()
        for dependency in package['dependencies']:
            if dependency['name'] in names:
                (dev if dependency.get('kind') == 'dev' else normal).add(dependency['name'])
        edges[package['name']] = (normal, dev)
    return edges


def dependents(changed, edges):
    """Changed crates, every crate whose library builds on one, and every crate whose tests use one."""
    library = set(changed)
    while True:
        more = {name for name, (normal, _) in edges.items() if normal & library} - library
        if not more:
            break
        library |= more
    return library | {name for name, (normal, dev) in edges.items() if (normal | dev) & library}


def selected(paths, every, edges):
    chosen = set()
    for path in paths:
        if path in WORKSPACE_FILES or path.startswith(WORKSPACE_DIRS):
            return list(every)
        if path.startswith(SHARED_TEST_DATA):
            chosen.add(TEST_DATA_CRATE)
        parts = path.split('/')
        if len(parts) > 2 and parts[0] == 'crates' and parts[1] in every:
            chosen.add(parts[1])
    return sorted(dependents(chosen, edges) & set(every))


def commands(names):
    library = [name for name in names if name != 'biomcp']
    runs = []
    if library:
        runs.append(['cargo', 'test', '--locked', '--no-fail-fast', '--features', 'biomcp-types/schema',
                     *[arg for name in library for arg in ('-p', name)]])
    if 'biomcp' in names:
        # The gate already runs biomcp's cases and ratchets targets. Its tools target is
        # test = false, so neither the gate nor library-tests runs it (deferred by 0921).
        runs.append(['cargo', 'test', '--locked', '--no-fail-fast', '-p', 'biomcp', '--bin', 'biomcp'])
    threads = os.environ.get('RUST_TEST_THREADS', '').strip()
    return [run + ['--', '--test-threads', threads] if threads else run for run in runs]


def main():
    every = crates()
    try:
        base = git('merge-base', 'HEAD', BASE_REF)
        # Names only, without rename detection, so a partial clone fetches no file contents.
        paths = git('diff', '--name-only', '--no-renames', base, 'HEAD').splitlines()
    except subprocess.CalledProcessError as error:
        print(f'library-tests: no base from {BASE_REF} ({error.stderr.strip()}); running every crate', flush=True)
        base, paths = 'none', None
    names = every
    if paths is not None:
        try:
            names = selected(paths, every, graph())
        except (subprocess.CalledProcessError, ValueError, KeyError) as error:
            print(f'library-tests: no crate graph from cargo metadata ({error}); running every crate', flush=True)
    print(f"library-tests base={base} crates={','.join(names) or 'none'}", flush=True)
    # Run every command, so one gate names every failing test.
    status = 0
    for command in commands(names):
        print('library-tests run: ' + ' '.join(command), flush=True)
        status = subprocess.run(command).returncode or status
    return status


if __name__ == '__main__':
    raise SystemExit(main())

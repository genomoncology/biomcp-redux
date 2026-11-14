"""Run the library tests of every crate the branch changed since it left main.

The landing gate runs this so a test added or changed under a ticket runs before
the ticket lands (0914). It names the base, the changed crates and each cargo
command in the log, and cargo names every test binary and test it runs.
A change to the workspace manifest, lock file or toolchain selects every crate.
When no base can be found it runs every crate rather than none.
"""
import os
import subprocess
import sys

BASE_REF = os.environ.get('LIBRARY_TESTS_BASE', 'origin/main')
WORKSPACE_FILES = ('Cargo.toml', 'Cargo.lock', 'rust-toolchain', 'rust-toolchain.toml')
WORKSPACE_DIRS = ('.cargo/',)


def git(*args):
    return subprocess.run(['git', *args], check=True, capture_output=True, text=True).stdout.strip()


def crates():
    return sorted(name for name in os.listdir('crates') if os.path.isfile(os.path.join('crates', name, 'Cargo.toml')))


def selected(paths, every):
    chosen = set()
    for path in paths:
        if path in WORKSPACE_FILES or path.startswith(WORKSPACE_DIRS):
            return list(every)
        parts = path.split('/')
        if len(parts) > 2 and parts[0] == 'crates' and parts[1] in every:
            chosen.add(parts[1])
    return sorted(chosen)


def commands(names):
    library = [name for name in names if name != 'biomcp']
    runs = []
    if library:
        runs.append(['cargo', 'test', '--locked', '--features', 'biomcp-types/schema',
                     *[arg for name in library for arg in ('-p', name)]])
    if 'biomcp' in names:
        # The biomcp crate's other test targets are cases and ratchets, which the gate already runs.
        runs.append(['cargo', 'test', '--locked', '-p', 'biomcp', '--bin', 'biomcp'])
    threads = os.environ.get('RUST_TEST_THREADS', '').strip()
    return [run + ['--', '--test-threads', threads] if threads else run for run in runs]


def main():
    every = crates()
    try:
        base = git('merge-base', 'HEAD', BASE_REF)
        # Names only, without rename detection, so a partial clone fetches no file contents.
        paths = git('diff', '--name-only', '--no-renames', base, 'HEAD').splitlines()
        names = selected(paths, every)
    except subprocess.CalledProcessError as error:
        print(f'library-tests: no base from {BASE_REF} ({error.stderr.strip()}); running every crate', flush=True)
        base, names = 'none', every
    print(f"library-tests base={base} crates={','.join(names) or 'none'}", flush=True)
    for command in commands(names):
        print('library-tests run: ' + ' '.join(command), flush=True)
        status = subprocess.run(command).returncode
        if status:
            return status
    return 0


if __name__ == '__main__':
    raise SystemExit(main())

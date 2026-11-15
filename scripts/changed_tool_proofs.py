"""Run the tooling proofs whose files, or the runner code they exercise, the branch changed.

The landing gate runs this so a broken proof fails the gate (0944). The whole catalog
takes about five minutes, so, as changed-library-tests does (0914/0921), it diffs HEAD
against its merge base with main. The proofs live in the case runner and test it, so a
change to the runner, its test support crate, the embedded schemas the schema proofs
compile, or the workspace build selects every proof. A change to a proof's own input
outside them selects that proof. Other changes run none. When no base can be found it
runs every proof rather than none. `make proof` still runs the whole catalog.
"""
import os
import subprocess
import sys

BASE_REF = os.environ.get('TOOL_PROOFS_BASE', 'origin/main')
WORKSPACE_FILES = ('Cargo.toml', 'Cargo.lock', 'rust-toolchain', 'rust-toolchain.toml')
# Any change under these selects the whole catalog.
RUNNER_INPUTS = ('.cargo/', 'crates/biomcp/tests/', 'crates/biomcp/Cargo.toml',
                 'crates/biomcp-test-support/', 'crates/biomcp-types/schemas/',
                 'scripts/changed_tool_proofs.py')
# A proof's own input outside the runner, and the name filter that selects its proofs.
PROOF_INPUTS = {'tests/replies/trial/gene/hgnc/': 'cache-resilience/hgnc-'}
EVERY = ''


def selected(paths):
    """Name filters to run: [EVERY] for the whole catalog, or none."""
    filters = set()
    for path in paths:
        if path in WORKSPACE_FILES or path.startswith(RUNNER_INPUTS):
            return [EVERY]
        filters.update(name for prefix, name in PROOF_INPUTS.items() if path.startswith(prefix))
    return sorted(filters)


def command(name):
    run = ['cargo', 'test', '--locked', '-p', 'biomcp', '--test', 'tools', '--', '--proofs']
    run += [name] if name else []
    threads = os.environ.get('RUST_TEST_THREADS', '').strip()
    return run + ['--test-threads', threads] if threads else run


def main():
    try:
        base = subprocess.run(['git', 'merge-base', 'HEAD', BASE_REF], check=True,
                              capture_output=True, text=True).stdout.strip()
        # Names only, without rename detection, so a partial clone fetches no file contents.
        paths = subprocess.run(['git', 'diff', '--name-only', '--no-renames', base, 'HEAD'], check=True,
                               capture_output=True, text=True).stdout.splitlines()
        names = selected(paths)
    except subprocess.CalledProcessError as error:
        print(f'tool-proofs: no base from {BASE_REF} ({error.stderr.strip()}); running every proof', flush=True)
        base, names = 'none', [EVERY]
    shown = ','.join('all' if name == EVERY else name for name in names) or 'none'
    print(f'tool-proofs base={base} proofs={shown}', flush=True)
    os.environ['BIOMCP_TEST_PYTHON'] = sys.executable
    # Run every command, so one gate names every failing proof.
    status = 0
    for name in names:
        run = command(name)
        print('tool-proofs run: ' + ' '.join(run), flush=True)
        status = subprocess.run(run).returncode or status
    return status


if __name__ == '__main__':
    raise SystemExit(main())

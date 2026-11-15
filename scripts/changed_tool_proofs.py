"""Run the tooling proofs whose inputs the branch changed since it left main.

The landing gate runs this so a broken proof fails the gate (0944). The whole catalog
takes about five minutes, so, as changed-library-tests does (0914/0921), it diffs HEAD
against its merge base with main and runs only the proofs those paths can change.
`make proof` and `make full-suite` still run the whole catalog.

Every proof, slow ones included, is selected by a change to the case runner and proof
code (crates/biomcp/tests/, crates/biomcp/Cargo.toml, crates/biomcp/build.rs,
crates/biomcp-test-support/), the Makefile that names the proof command, this script, or
the workspace manifest, lock file, toolchain or .cargo/.

Every proof except the slow ones (SLOW) is selected by a change to product code:
crates/biomcp/src/, crates/biomcp-types/, crates/biomcp-query/, crates/biomcp-fetch/ and
crates/biomcp-adapters/. Every proof group runs it (mcp-http, schema-discovery,
cache-resilience, article-semantic-sections, and runner/report-parity through
biomcp-types), and which adapters a proof reaches is not listed by hand, because such a
list drifts.

Files outside product code that a proof reads, or that the binary's build script embeds,
add the proofs that read them (PROOF_INPUTS): recorded HGNC replies, the recorded
article replies, the CIViC fixtures, and skills/, tests/cases/ and the embedded docs.
Every other path, such as other tests/cases/ and tests/replies/ data, docs and scripts,
selects no proof. When no base can be found it runs every proof rather than none.
"""
import os
import subprocess
import sys

BASE_REF = os.environ.get('TOOL_PROOFS_BASE', 'origin/main')
EVERY = ''
PRODUCT = 'all but the slow proofs'
WORKSPACE_FILES = ('Cargo.toml', 'Cargo.lock', 'rust-toolchain', 'rust-toolchain.toml', 'Makefile')
EVERY_INPUTS = ('.cargo/', 'crates/biomcp/tests/', 'crates/biomcp/Cargo.toml', 'crates/biomcp/build.rs',
                'crates/biomcp-test-support/', 'scripts/changed_tool_proofs.py')
PRODUCT_INPUTS = ('crates/biomcp/src/', 'crates/biomcp-types/', 'crates/biomcp-query/', 'crates/biomcp-fetch/',
                  'crates/biomcp-adapters/')
# Proofs over 30 s alone, which product changes do not select (0944, measured in j41018):
# rejects-bad-cases took 264 s and selects-and-lists-cases 75 s; the rest took 15 s or less.
# Speeding them up is a follow-up ticket, filed by the PM from 0944; until then they run
# when runner, proof or build files change and in `make proof` and `make full-suite`.
SLOW = ('schema-discovery/rejects-bad-cases', 'schema-discovery/selects-and-lists-cases')
HELP = 'schema-discovery/catalog-and-help-reply-discovery'
NATIVE = 'schema-discovery/persistent-native-evidence'
# Files a proof reads, or the binary's build script embeds, and the proofs they select.
PROOF_INPUTS = {
    'tests/replies/trial/gene/hgnc/': ('cache-resilience/hgnc-',),
    'tests/replies/article/edges/live/': ('article-semantic-sections/',),
    'tests/fixtures/drug/civic/': (NATIVE,),
    'skills/': (HELP,), 'tests/cases/': (HELP,), 'docs/reference/commands.md': (HELP,),
    'docs/reference/source-pacing.md': (HELP,), 'docs/recipes.md': (HELP,),
}


def proofs(path):
    """Name filters one changed path selects."""
    if path in WORKSPACE_FILES or path.startswith(EVERY_INPUTS):
        return (EVERY,)
    if path.startswith(PRODUCT_INPUTS):
        return (PRODUCT,)
    return tuple(name for prefix, names in PROOF_INPUTS.items() if path.startswith(prefix) for name in names)


def selected(paths):
    """Name filters to run: [EVERY], [PRODUCT] plus any narrower filter naming a slow proof,
    or the narrower filters the paths select."""
    filters = {name for path in paths for name in proofs(path)}
    if EVERY in filters:
        return [EVERY]
    if PRODUCT in filters:
        return [PRODUCT] + sorted(name for name in filters - {PRODUCT}
                                  if any(name in slow for slow in SLOW))
    return sorted(filters)


def command(name):
    run = ['cargo', 'test', '--locked', '-p', 'biomcp', '--test', 'tools', '--', '--proofs']
    if name == PRODUCT:
        run += [argument for slow in SLOW for argument in ('--skip', slow)]
    elif name:
        run.append(name)
    threads = os.environ.get('RUST_TEST_THREADS', '').strip()
    return run + ['--test-threads', threads] if threads else run


def main():
    try:
        base = subprocess.run(['git', 'merge-base', 'HEAD', BASE_REF], check=True,
                              capture_output=True, text=True).stdout.strip()
        # NUL-separated names keep non-ASCII paths unquoted; no rename detection, so a
        # partial clone fetches no file contents.
        listed = subprocess.run(['git', 'diff', '-z', '--name-only', '--no-renames', base, 'HEAD'],
                                check=True, capture_output=True).stdout
        names = selected(os.fsdecode(path) for path in listed.split(b'\0') if path)
    except subprocess.CalledProcessError as error:
        detail = os.fsdecode(error.stderr or b'').strip()
        print(f'tool-proofs: no base from {BASE_REF} ({detail}); running every proof', flush=True)
        base, names = 'none', [EVERY]
    shown = ','.join({EVERY: 'all', PRODUCT: 'all-but-slow'}.get(name, name) for name in names) or 'none'
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

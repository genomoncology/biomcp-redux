"""Run the tooling proofs whose inputs the branch changed since it left main.

The landing gate runs this so a broken proof fails the gate (0944). The whole catalog
takes about five minutes, so, as changed-library-tests does (0914/0921), it diffs HEAD
against its merge base with main and runs only the proofs those paths can change.
`make proof` and `make full-suite` still run the whole catalog.

Every proof is selected by a change to:
- the case runner and proof code (crates/biomcp/tests/, crates/biomcp/Cargo.toml,
  crates/biomcp-test-support/), the embedded schemas the schema proofs compile
  (crates/biomcp-types/schemas/), the Makefile that names the proof command, this script,
  or the workspace manifest, lock file, toolchain or .cargo/;
- the product code every proof group runs: crates/biomcp/src/ (the binary and its MCP
  server, run by the mcp-http, schema-discovery, cache-resilience and article proofs),
  crates/biomcp-types/src/ (also read by runner/report-parity), crates/biomcp-query/
  (source terms, requests and the HGNC query) and crates/biomcp-fetch/ (the fetch layer
  the HGNC and fixture-evidence proofs call directly);
- shared adapter code: crates/biomcp-adapters/ outside a single source's module.

Some proofs are selected by one source's adapter or recorded data alone (ADAPTER_PROOFS,
PROOF_INPUTS), and the help discovery proof by the files the binary's build script embeds.
Any other single-source adapter module (OTHER_SOURCE_ADAPTERS), and every other path,
such as other tests/cases/ and tests/replies/ data, docs and scripts, selects no proof.
When no base can be found it runs every proof rather than none.
"""
import os
import subprocess
import sys

BASE_REF = os.environ.get('TOOL_PROOFS_BASE', 'origin/main')
EVERY = ''
WORKSPACE_FILES = ('Cargo.toml', 'Cargo.lock', 'rust-toolchain', 'rust-toolchain.toml', 'Makefile')
EVERY_INPUTS = ('.cargo/', 'crates/biomcp/tests/', 'crates/biomcp/Cargo.toml', 'crates/biomcp/build.rs',
                'crates/biomcp/src/', 'crates/biomcp-test-support/', 'crates/biomcp-types/',
                'crates/biomcp-query/', 'crates/biomcp-fetch/', 'scripts/changed_tool_proofs.py')
ADAPTERS = 'crates/biomcp-adapters/src/'
HELP = 'schema-discovery/catalog-and-help-reply-discovery'
NATIVE = 'schema-discovery/persistent-native-evidence'
# A source adapter module and the proofs that run it.
ADAPTER_PROOFS = {'hgnc': ('cache-resilience/hgnc-',),
                  'semantic_scholar': ('article-semantic-sections/',),
                  'pubmed': ('article-semantic-sections/',), 'civic': (NATIVE,)}
# Single-source adapter modules no proof runs; any other adapter path is shared code.
OTHER_SOURCE_ADAPTERS = frozenset('''
    alphagenome mygene oncokb complex_portal gnomad interpro uniprot hpo reactome wikipathways
    clingen gencc hpa europepmc pubtator pmc_oa icite litvar orcid litsense allele_registry ctgov
    mutalyzer myvariant nci variantvalidator cbioportal clinvar erepo hotspots gwas chembl ddinter
    drugsfda ema mychem openfda_label opentargets monarch mydisease ols orphadata seer faers maude
    fda_cdx fda_pma gtr who_ivd clinpgx cpic fda_510k who_pq vaers cellosaurus fda_device_recall
    fda_drug_recall'''.split())
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
    if path.startswith('crates/biomcp-adapters/'):
        name = path[len(ADAPTERS):].split('/')[0].removesuffix('.rs') if path.startswith(ADAPTERS) else ''
        if name in OTHER_SOURCE_ADAPTERS:
            return ()
        return ADAPTER_PROOFS.get(name, (EVERY,))
    return tuple(name for prefix, names in PROOF_INPUTS.items() if path.startswith(prefix) for name in names)


def selected(paths):
    """Name filters to run: [EVERY] for the whole catalog, or the filters the paths select."""
    filters = {name for path in paths for name in proofs(path)}
    return [EVERY] if EVERY in filters else sorted(filters)


def command(name):
    run = ['cargo', 'test', '--locked', '-p', 'biomcp', '--test', 'tools', '--', '--proofs']
    run += [name] if name else []
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

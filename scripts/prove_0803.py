"""One-off scoped witnesses for the three meaningful 0803 survivors; do not land."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
PRODUCT = '34d6e02d1ebeac4d55e44e05099854f15e718d57'
CASES = [
    ('crates/biomcp-query/src/source_snapshot.rs', '!self.2', 'false', [
        ('protein-sections/displayed-bounds-and-identity', 'healthy protein window was rebuilt'),
    ]),
    ('crates/biomcp-query/src/author/mod.rs',
     'self.pages.iter().all(|(_, refused)| !refused)\n'
     '            && crate::source_snapshot::sources_whole(std::slice::from_ref(&self.source))',
     'false', [('author-search/healthy-window-reuses-pool', 'retained author window changed')]),
    ('crates/biomcp-query/src/author/mod.rs',
     'self.pages.iter().all(|(_, refused)| !refused)\n'
     '            && crate::source_snapshot::sources_whole(std::slice::from_ref(&self.source))',
     'true', [
         ('author-search/refused-window-is-explicit-only', 'incomplete author pool became the implicit default'),
         ('author-search/refused-extension-retires-default', 'refused extension remained the implicit default'),
     ]),
]


def main():
    print('Supplementary test-driver source:', subprocess.check_output(
        ['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(), flush=True)
    print('Selected product:', PRODUCT, flush=True)
    changed = subprocess.check_output(
        ['git', 'diff', '--name-only', PRODUCT, 'HEAD', '--', 'crates', 'Cargo.toml',
         'Cargo.lock', 'rust-toolchain.toml', 'release/toolchain.json'], cwd=ROOT, text=True)
    unexpected = [p for p in changed.splitlines() if '/tests/' not in p]
    if unexpected:
        raise ValueError('Unmutated production differs from selected product: ' + repr(unexpected))
    print('Unmutated production equivalence: PASS', flush=True)
    for command in [
        ['cargo', 'fmt', '--all', '--', '--check'],
        ['cargo', 'clippy', '--locked', '-p', 'biomcp', '--test', 'tools', '--', '-D', 'warnings'],
        ['cargo', 'test', '--locked', '-p', 'biomcp', '--test', 'ratchets'],
    ]:
        subprocess.run(command, cwd=ROOT, check=True)
    print('Unmutated formatting, tools lint and ratchets: PASS', flush=True)
    for relative, before, replacement, trials in CASES:
        path = ROOT / relative
        original = path.read_text()
        if original.count(before) != 1:
            raise ValueError('Replacement must match exactly once: ' + relative)
        try:
            path.write_text(original.replace(before, replacement))
            print(f'REPLACEMENT {relative}: {before!r} -> {replacement}', flush=True)
            for trial, witness in trials:
                result = subprocess.run(
                    [sys.executable, 'scripts/trials.py', '--target', 'tools', '--proofs', trial],
                    cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
                print(result.stdout, end='', flush=True)
                if result.returncode == 0 or witness not in result.stdout:
                    raise ValueError('Missing actual public-command red witness: ' + trial)
                print('CONFIRMED RED:', trial, flush=True)
        finally:
            path.write_text(original)
    print('All three scoped replacements killed by four public-command checks', flush=True)


if __name__ == '__main__':
    main()

"""Check Author and ORCID declarations against recorded source inputs."""
import json
from pathlib import Path
import subprocess
import sys

from source_capture import outside_git
from test_parity_batches import execute, selection, verdict, witnesses
from test_parity_runner import ROOT


def inventory(out):
    destination = out / 'inventory.json'
    result = subprocess.run(
        ['make', 'parity-inventory',
         'PLAN=' + str(ROOT / 'tests/parity/inventory.json'),
         'OUT=' + str(destination)], cwd=ROOT, capture_output=True)
    (out / 'inventory.log').write_bytes(result.stdout + result.stderr)
    report = json.loads(destination.read_text())
    rows = [row for row in report['cases']
            if row['case'].startswith(('author/', 'orcid/'))]
    assert report['unsafe_plans'] == 0, report
    assert not any(row['class'] == 'pending_family' for row in rows), rows
    assert not any(row.get('gaps', 0) for row in rows), rows
    assert sum(row['class'] == 'direct' for row in rows) == 18, rows
    assert sum(row['class'] == 'unprojectable' for row in rows) == 10, rows


def main():
    baseline, out = map(Path, sys.argv[1:])
    outside_git(out)
    out.mkdir()
    inventory(out)
    witnesses('author-orcid.json', 'author/empty.yaml', '/page/total',
              '/providers/0/results', 'author-empty', baseline, out)
    row = selection('author-orcid.json', 'author/literal-whitespace-query.yaml')
    report, result = execute(row, 'author-whitespace', baseline, out)
    assert report['assessment_gaps'] == 0, report
    for pointer in ('/result/items', '/result/matching_records'):
        fact = verdict(result, pointer)
        assert fact['baseline']['verdict'] == 'right', fact
        assert fact['candidate']['verdict'] == 'right', fact
    query = verdict(result, '/query/query')
    assert query['baseline']['verdict'] == 'unprojectable', query
    sys.stdout.write('Author and ORCID inventory and both verdicts passed: '
                     + str(out) + '\n')


if __name__ == '__main__':
    main()

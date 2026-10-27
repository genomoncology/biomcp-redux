"""Require every declared indexing question and each reviewed sequence assertion offline."""
import copy
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
NAMES = {
    'article/indexing/own-record.yaml',
    'article/semantic-scholar/synthetic-minimal.yaml',
    'author/papers-bad-id.yaml',
    'author/papers-wrong-offset.yaml',
    'orcid/wrong-works.yaml',
    'protein/wrong-accession.yaml',
    'protein/wrong-taxon.yaml',
}


def main():
    output = Path(sys.argv[1])
    assert output.is_absolute() and not output.exists()
    subprocess.run(['cargo', 'test', '--locked', '-p', 'biomcp', '--test', 'cases',
                    '--', '--parity-inventory', str(ROOT / 'tests/parity/inventory.json'),
                    str(ROOT / 'tests/cases'), str(output)], cwd=ROOT, check=False)
    inventory = json.loads(output.read_text())
    selected = [row for row in inventory['cases'] if row['case'] in NAMES]
    assert len(selected) == len(NAMES)
    for row in selected:
        assert not row.get('missing_projections'), row
        assert row.get('unsafe_reason') is None, row
        assert row.get('declaration_reuse', {}).get('gaps', 0) == 0, row
    assert not any(row.get('missing_projections') for row in inventory['cases'])
    assert 'sequence must assess every harvested assertion' not in output.read_text()
    if len(sys.argv) > 2:
        from test_parity_runner import run
        baseline, evidence = Path(sys.argv[2]), Path(sys.argv[3])
        declarations = [row for batch in (ROOT / 'tests/parity/batches').glob('*.json')
                        for row in json.loads(batch.read_text()).get('cases', [])]
        sequences = []
        for name in sorted(NAMES - {'article/indexing/own-record.yaml'}):
            template, = [row for row in declarations if row['case'] == name]
            case = json.loads((ROOT / 'tests/cases' / name).read_text())
            steps = []
            for index, step in enumerate(case['steps']):
                row = copy.deepcopy(template)
                row['step'] = index
                for field in ('facts', 'unprojectable_facts'):
                    row[field] = {p: value for p, value in row[field].items() if p in step['expect']}
                steps.append(row)
            sequences.append({'case': name, 'steps': steps})
        plan = output.with_name(output.stem + '-sequences.json')
        plan.write_text(json.dumps({'baseline_release': '0.9.3', 'cases': [], 'sequences': sequences}))
        report = run(plan, baseline, evidence, True)
        assert report['assessment_gaps'] == report['regressions'] == 0, report
        assert len(report['cases']) == 12, report
    print('All indexing pointers and six sequence assertion declarations are complete:', output)


if __name__ == '__main__':
    main()

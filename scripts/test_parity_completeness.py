"""Require every declared indexing question and each reviewed sequence assertion offline."""
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
    print('All indexing pointers and six sequence assertion declarations are complete:', output)


if __name__ == '__main__':
    main()

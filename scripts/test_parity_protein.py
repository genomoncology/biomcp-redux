"""Prove Protein declarations with retained replies and both actual executables."""
import json
from pathlib import Path
import subprocess
import sys

from source_capture import outside_git
from test_parity_batches import execute, selection, verdict
from test_parity_runner import ROOT


def inventory(out):
    destination = out / 'inventory.json'
    result = subprocess.run(['make', 'parity-inventory',
                             'PLAN=' + str(ROOT / 'tests/parity/inventory.json'),
                             'OUT=' + str(destination)], cwd=ROOT, capture_output=True)
    (out / 'inventory.log').write_bytes(result.stdout + result.stderr)
    report = json.loads(destination.read_text())
    declarations = json.loads((ROOT / 'tests/parity/batches/protein.json').read_text())
    names = {row['case'] for row in declarations['cases']}
    names.update(row['case'] for row in declarations['inventory']['unprojectable'])
    rows = [row for row in report['cases'] if row['case'] in names]
    assert len(rows) == 7
    assert all(row['gaps'] == 0 and not row.get('unsafe_reason') for row in rows), rows
    assert sum(row['class'] == 'direct' for row in rows) == 5
    assert sum(row['class'] == 'unprojectable' for row in rows) == 2
    assert report['unsafe_plans'] == 0
    assert (result.returncode == 0) == (report['assessment_gaps'] == 0)
    return {'protein_cases': len(rows), 'protein_gaps': 0, 'unsafe_plans': 0}


def source_evidence(out, name, result):
    source = json.loads((out / name / '0-source-0.json').read_text())
    exchange = source['capture']['exchanges'][source['exchange']]
    assert source['status'] == exchange['status']
    assert source['adapted_body'] == exchange['body']['value']
    requests = json.loads((out / name / '0-baseline-requests.json').read_text())
    assert requests and result['baseline']['recorded_requests'] == len(requests)
    assert all(host == 'rest.uniprot.org' and
               request.splitlines()[0] == 'GET /uniprotkb/P00533.json HTTP/1.1'
               for host, request in requests), requests
    for label in ('baseline', 'candidate'):
        command = result[label]
        raw = (out / name / command['stdout']).read_bytes()
        assert json.loads(raw) == command['parsed']
    return exchange


def main():
    baseline, out = map(Path, sys.argv[1:])
    outside_git(out)
    out.mkdir()
    summary = inventory(out)
    row = selection('protein.json', 'protein/rate-limit.yaml')
    report, result = execute(row, 'rate-limit', baseline, out)
    fact = verdict(result, '/sources/0/source')
    assert report['assessment_gaps'] == 0
    assert result['baseline']['exit'] == 1 and result['candidate']['exit'] == 4
    assert fact['baseline']['verdict'] == fact['candidate']['verdict'] == 'right', fact
    assert fact['baseline']['actual'] == fact['candidate']['actual'] == 'uniprot'
    assert result['baseline']['parsed']['error']['source'] == 'UniProt'
    exchange = source_evidence(out, 'rate-limit', result)
    assert exchange['status'] == 429 and exchange['delay_ms'] == 0
    row['facts']['/sources/0/source'] = '/error/code'
    report, result = execute(row, 'wrong-pointer', baseline, out)
    fact = verdict(result, '/sources/0/source')
    assert fact['baseline']['verdict'] == 'wrong', fact
    assert fact['candidate']['verdict'] == 'right', fact
    assert fact['baseline']['actual'] == result['baseline']['parsed']['error']['code']
    source_evidence(out, 'wrong-pointer', result)
    row = selection('protein.json', 'protein/timeout.yaml')
    report, result = execute(row, 'timeout', baseline, out)
    fact = verdict(result, '/sources/0/source')
    assert report['assessment_gaps'] == 0
    assert result['baseline']['exit'] == 0 and result['candidate']['exit'] == 4
    assert fact['baseline']['verdict'] == fact['candidate']['verdict'] == 'right', fact
    assert result['baseline']['parsed']['_meta']['section_sources'][0]['sources'][0] == 'UniProt'
    exchange = source_evidence(out, 'timeout', result)
    assert exchange['status'] == 200 and exchange['delay_ms'] == 9000
    assert result['baseline']['elapsed_us'] >= exchange['delay_ms'] * 1000
    summary.update(positive=['right', 'right'], wrong_pointer=['wrong', 'right'],
                   baseline_exit=1, candidate_exit=4, evidence=str(out))
    sys.stdout.write(json.dumps(summary) + '\n')


if __name__ == '__main__':
    main()

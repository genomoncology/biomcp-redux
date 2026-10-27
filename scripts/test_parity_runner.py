"""Prove the recorded sample against both real binaries and retain raw evidence.

Run separately from the routine suite: python3 scripts/test_parity_runner.py
/absolute/released-0.9.3 /absolute/new-evidence-root. Prints one summary.
"""
import json
from pathlib import Path
import subprocess
import sys

from source_capture import outside_git, write_json

ROOT = Path(__file__).resolve().parents[1]


def run(plan, baseline, out, expected):
    result = subprocess.run(['make', 'parity-run', 'PLAN=' + str(plan),
                             'BASELINE=' + str(baseline), 'OUT=' + str(out)],
                            cwd=ROOT, capture_output=True)
    (out.parent / (out.name + '.log')).write_bytes(result.stdout + result.stderr)
    assert (result.returncode == 0) == expected, result.stderr.decode(errors='replace')
    report = json.loads((out / 'report.json').read_text())
    row = report['cases'][0]
    for label in ('baseline', 'candidate'):
        command = row[label]
        raw = (out / command['stdout']).read_bytes()
        assert len(raw) == command['stdout_bytes'] > 0
        assert len((out / command['stderr']).read_bytes()) == command['stderr_bytes']
        assert command['elapsed_us'] > 0 and command['exit'] == 0
        assert json.loads(raw) == command['parsed']
    return report


def main():
    baseline, out = map(Path, sys.argv[1:])
    outside_git(out)
    out.mkdir()
    sample = ROOT / 'tests/parity/sample.json'
    report = run(sample, baseline, out / 'sample', True)
    row = report['cases'][0]
    fact, = row['facts']
    assert fact['baseline']['verdict'] == 'wrong'
    assert fact['candidate']['verdict'] == 'right'
    assert fact['baseline']['actual'] == row['baseline']['parsed']['summary']
    assert fact['candidate']['actual'] == row['candidate']['parsed']['result']['items'][0]['answer']['gene']['summary']['text']
    assert fact['baseline']['actual'] != fact['candidate']['actual']
    assert row['baseline']['command'][-3:] == ['gene', 'definition', 'BRAF']
    assert row['candidate']['command'][:3] == ['get', 'gene', 'BRAF']
    source = json.loads((out / 'sample/0-source-0.json').read_text())
    assert source['adapted_body']['hits'] == source['capture']['exchanges'][0]['body']['value']
    requests = json.loads((out / 'sample/0-baseline-requests.json').read_text())
    assert len(requests) == row['baseline']['recorded_requests'] == 1
    assert requests[0][0].startswith('CONNECT mygene.info:443 ')
    plan = json.loads(sample.read_text())
    plan['cases'][0]['facts'] = {}
    missing = out / 'missing-projection.json'
    write_json(missing, plan)
    gap = run(missing, baseline, out / 'gap', False)
    assert gap['assessment_gaps'] == 1 and gap['regressions'] == 0
    fact, = gap['cases'][0]['facts']
    assert fact['baseline']['verdict'] == fact['candidate']['verdict'] == 'assessment_gap'
    assert 'missing baseline projection' in fact['baseline']['reason']
    print('Recorded parity sample and missing-projection refusal passed; evidence: ' + str(out))


if __name__ == '__main__':
    main()

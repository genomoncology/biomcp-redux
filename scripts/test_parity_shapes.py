"""Exercise value binding against original Pathway recordings and the actual native binary.

The temporary Pathway declarations prove the shared matcher. They add no family coverage.
"""
import copy
import json
import os
from pathlib import Path
import sys

from source_capture import outside_git, write_json
from test_parity_runner import ROOT, run


def main():
    baseline, out = map(Path, sys.argv[1:])
    outside_git(out)
    out.mkdir()
    fixtures = out / 'fixtures'
    (fixtures / 'cases/pathway').mkdir(parents=True)
    (fixtures / 'replies').symlink_to(ROOT / 'tests/replies', target_is_directory=True)
    (fixtures / 'parity').symlink_to(ROOT / 'tests/parity', target_is_directory=True)
    plan = json.loads((ROOT / 'tests/parity/batches/pathway.json').read_bytes())
    names = ['pathway/card-R-HSA-157118.yaml', 'pathway/card-R-HSA-5673001.yaml']
    names = [name for name in names if any(row['case'] == name for row in plan['cases'])]
    assert names
    sequences = []
    for name in names:
        case = json.loads((ROOT / 'tests/cases' / name).read_bytes())
        assert len(case['steps']) == 1
        case['steps'].append(copy.deepcopy(case['steps'][0]))
        write_json(fixtures / 'cases' / name, case)
        sequences.append({'case': name, 'steps': []})
    declaration = copy.deepcopy(next(row for row in plan['cases'] if row['case'] == names[0]))
    identifier = declaration['baseline_command'][2]
    declaration['baseline_command'][2] = '{identifier}'
    for route in declaration['recordings']:
        route['capture'] = ''
        route['request_prefix'] = route['request_prefix'].replace(identifier, '{identifier}')
    shape = {'name': 'test-only-native-pathway', 'command': ['get', 'pathway', '{identifier}'],
             'exit': 0, 'config': None, 'declaration': declaration}
    environment = dict(os.environ, BIOMCP_TEST_FIXTURES=str(fixtures))
    for label in ['native', 'wrong']:
        if label == 'wrong':
            declaration['facts']['/result/pathway/name'] = '/id'
        path = out / (label + '.json')
        write_json(path, {'baseline_release': '0.9.3', 'cases': [],
                         'shapes': [shape], 'sequences': sequences})
        report = run(path, baseline, out / label, True, env=environment)
        assert report['assessment_gaps'] == report['regressions'] == 0
        assert len(report['cases']) == 2 * len(names)
        for row in report['cases']:
            fact = next(f for f in row['facts'] if f['pointer'] == '/result/pathway/name')
            assert fact['candidate']['verdict'] == 'right', fact
            assert fact['baseline']['verdict'] == ('right' if label == 'native' else 'wrong'), fact
            assert row['baseline']['recorded_requests'] >= 1
            assert row['baseline']['command'][-1] == row['candidate']['command'][2]
    print(json.dumps({'native_cases': names, 'proof': 'current-case native positive and wrong projection',
                      'evidence': str(out)}))


if __name__ == '__main__':
    main()

"""Exercise reviewed Trial and Pathway mappings through both real executables."""
import copy
import json
from pathlib import Path
import subprocess
import sys

from source_capture import outside_git, write_json
from test_parity_runner import ROOT, reject, run


def selection(batch, case):
    declarations = json.loads((ROOT / 'tests/parity/batches' / batch).read_text())
    return copy.deepcopy(next(row for row in declarations['cases'] if row['case'] == case))


def execute(row, name, baseline, out, success=True):
    plan = out / (name + '.json')
    write_json(plan, {'baseline_release': '0.9.3', 'cases': [row]})
    report = run(plan, baseline, out / name, success)
    assert report['regressions'] == 0
    result, = report['cases']
    assert result['candidate']['provider_connections'] == 0
    return report, result


def verdict(row, pointer):
    return next(fact for fact in row['facts'] if fact['pointer'] == pointer)


def witnesses(batch, case, pointer, wrong, label, baseline, out):
    row = selection(batch, case)
    report, result = execute(row, label, baseline, out)
    fact = verdict(result, pointer)
    assert fact['baseline']['verdict'] == fact['candidate']['verdict'] == 'right', fact
    assert report['assessment_gaps'] == 0
    row['facts'][pointer] = wrong
    report, result = execute(row, label + '-wrong', baseline, out)
    fact = verdict(result, pointer)
    assert fact['baseline']['verdict'] == 'wrong', fact
    assert fact['candidate']['verdict'] == 'right', fact
    assert report['assessment_gaps'] == 0


def guards(baseline, out):
    row = selection('trial.json', 'trial/phase.yaml')
    row['recordings'][0]['request_query']['query.term'] = 'AREA[Phase]PHASE2'
    report, result = execute(row, 'query-mismatch', baseline, out, False)
    assert report['assessment_gaps'] > 0
    assert all(f['baseline']['verdict'] in ('assessment_gap', 'unprojectable') for f in result['facts'])
    row = selection('pathway.json', 'pathway/card-R-HSA-157118.yaml')
    pointer = '/result/pathway/id'
    row['facts'][pointer]['object']['id']['pointer'] = '/section_outcomes'
    report, result = execute(row, 'identity-type', baseline, out, False)
    fact = verdict(result, pointer)
    assert fact['baseline']['verdict'] == 'assessment_gap'
    assert 'requires a string' in fact['baseline']['reason']
    row = selection('trial.json', 'trial/age-missing.yaml')
    row['baseline_command'].remove('--age')
    row['baseline_command'].remove('18')
    reject({'baseline_release': '0.9.3', 'cases': [row]}, baseline, out,
           'age-selector', 'must preserve harvested --age')


def main():
    baseline, out = map(Path, sys.argv[1:])
    outside_git(out)
    out.mkdir()
    inventory(out)
    witnesses('trial.json', 'trial/documents.yaml',
              '/result/items/0/sections/documents/data/items/1/filename',
              '/nct_id', 'trial', baseline, out)
    witnesses('pathway.json', 'pathway/card-R-HSA-157118.yaml',
              '/result/pathway/name', '/id', 'pathway', baseline, out)
    row = selection('pathway.json', 'pathway/failure-broken.yaml')
    report, result = execute(row, 'pathway-error', baseline, out)
    fact = verdict(result, '/sources/0/source')
    assert fact['baseline']['verdict'] == fact['candidate']['verdict'] == 'right', fact
    assert result['baseline']['exit'] == 1 and result['candidate']['exit'] == 4
    assert result['baseline']['json_stream'] == 'stdout'
    source = json.loads((out / 'pathway-error/0-source-0.json').read_text())
    assert source['status'] == 200 and isinstance(source['adapted_body'], str)
    assert source['adapted_body'] == source['capture']['exchanges'][source['exchange']]['body']['value']
    guards(baseline, out)
    sys.stdout.write('Trial and Pathway positive, wrong-value, source-error and guard witnesses passed: ' + str(out) + '\n')


def inventory(out):
    destination = out / 'inventory.json'
    result = subprocess.run(['make', 'parity-inventory',
                             'PLAN=' + str(ROOT / 'tests/parity/inventory.json'),
                             'OUT=' + str(destination)], cwd=ROOT, capture_output=True)
    (out / 'inventory.log').write_bytes(result.stdout + result.stderr)
    report = json.loads(destination.read_text())
    classes = report['classes']
    assert report['unsafe_plans'] == classes.get('unclassified', 0) == 0, classes
    assert classes['pending_family'] == report['assessment_gaps'] > 0
    assert result.returncode != 0, 'pending declarations must hold the whole inventory gate'
    owners = {}
    for row in report['cases']:
        if row['class'] == 'pending_family':
            owners[row['ticket']] = owners.get(row['ticket'], 0) + 1
    assert set(owners) == {'0300', '0301', '0302', '0303', '0304', '0305'}, owners
    sys.stdout.write(json.dumps({'dry_classes': classes, 'pending_owners': owners}) + '\n')


if __name__ == '__main__':
    main()

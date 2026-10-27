"""Prove the recorded sample against both real binaries and retain raw evidence.

Run separately from the routine suite: python3 scripts/test_parity_runner.py
/absolute/released-0.9.3 /absolute/new-evidence-root. Prints one summary.
"""
import copy
import json
import os
import shutil
from pathlib import Path
import subprocess
import sys

from source_capture import outside_git, write_json

ROOT = Path(__file__).resolve().parents[1]


def run(plan, baseline, out, expected, env=None):
    result = subprocess.run(['make', 'parity-run', 'PLAN=' + str(plan),
                             'BASELINE=' + str(baseline), 'OUT=' + str(out)],
                            cwd=ROOT, capture_output=True, env=env)
    (out.parent / (out.name + '.log')).write_bytes(result.stdout + result.stderr)
    assert (result.returncode == 0) == expected, result.stderr.decode(errors='replace')
    report = json.loads((out / 'report.json').read_text())
    row = report['cases'][0]
    for label in ('baseline', 'candidate'):
        command = row[label]
        if 'stdout' not in command:
            assert label == 'baseline' and set(command) == {'assessment_error'}, command
            assert command['assessment_error'], 'an unlaunched baseline needs its assessment gap'
            continue
        raw = (out / command['stdout']).read_bytes()
        assert len(raw) == command['stdout_bytes'] > 0
        assert len((out / command['stderr']).read_bytes()) == command['stderr_bytes']
        assert command['elapsed_us'] > 0
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
    assert row['candidate']['provider_connections'] == 0
    assert row['baseline']['exit'] == row['candidate']['exit'] == 0
    assert row['baseline']['command'][-3:] == ['gene', 'definition', 'BRAF']
    assert row['candidate']['command'][:3] == ['get', 'gene', 'BRAF']
    source = json.loads((out / 'sample/0-source-0.json').read_text())
    assert source['adapted_body']['hits'] == source['capture']['exchanges'][0]['body']['value']
    requests = json.loads((out / 'sample/0-baseline-requests.json').read_text())
    assert len(requests) == row['baseline']['recorded_requests'] == 1
    assert requests[0][0] == 'mygene.info'
    assert source['adapted_body']['total'] == len(source['adapted_body']['hits'])
    assert 'mygene-info' not in row['baseline']['disabled_sources'].split(',')
    assert 'opentargets' in row['baseline']['disabled_sources'].split(',')
    plan = json.loads(sample.read_text())
    plan['cases'][0]['facts'] = {}
    missing = out / 'missing-projection.json'
    write_json(missing, plan)
    gap = run(missing, baseline, out / 'gap', False)
    assert gap['assessment_gaps'] == 1 and gap['regressions'] == 0
    fact, = gap['cases'][0]['facts']
    assert fact['baseline']['verdict'] == fact['candidate']['verdict'] == 'assessment_gap'
    assert 'missing baseline projection' in fact['baseline']['reason']
    multi = copy.deepcopy(plan)
    multi['cases'][0]['assertion_pointers'] = ['/sources/*/source']
    write_json(out / 'multi.json', multi)
    expanded = run(out / 'multi.json', baseline, out / 'multi', False)
    assert expanded['assessment_gaps'] == 2
    assert len(expanded['cases'][0]['facts']) == 2
    separator = copy.deepcopy(plan)
    grammar = json.loads((ROOT / 'tests/cases/gene/sections/grammar.yaml').read_text())
    ordinal, step = next((i, step) for i, step in enumerate(grammar['steps']) if '--' in step.get('run', []))
    selection = separator['cases'][0]
    selection.update(case='gene/sections/grammar.yaml', step=ordinal, facts={}, assertion_pointers=['/query/input'])
    selection['recordings'][0]['capture'] = '../main/' + selection['recordings'][0]['capture']
    write_json(out / 'separator.json', separator)
    separated = run(out / 'separator.json', baseline, out / 'separator', False)
    command = separated['cases'][0]['candidate']
    assert command['parsed']['query']['input'] == step['expect']['/query/input']
    assert command['command'].index('--replay') < command['command'].index('--')
    assert command['provider_connections'] == 0
    repeated_disable = json.loads(sample.read_text())
    repeated_disable['cases'][0]['baseline_globals'].extend(['--disable-sources', 'opentargets'])
    reject(repeated_disable, baseline, out, 'disable', 'source disables are derived from routes')
    for name, arguments in [('command-disable', ['--disable-sources', 'opentargets']),
                            ('command-disable-inline', ['--disable-sources=opentargets'])]:
        command_disable = json.loads(sample.read_text())
        command_disable['cases'][0]['baseline_command'].extend(arguments)
        reject(command_disable, baseline, out, name, 'source disables are derived from routes')
    wrong_host = json.loads(sample.read_text())
    wrong_host['cases'][0]['recordings'][0]['host'] = 'unrelated.invalid'
    reject(wrong_host, baseline, out, 'host', 'does not belong to source')
    wrong_endpoint = json.loads(sample.read_text())
    wrong_endpoint['cases'][0]['recordings'][0]['endpoint_env'] = 'BIOMCP_OPENTARGETS_BASE'
    reject(wrong_endpoint, baseline, out, 'endpoint', 'does not belong to source')
    installed = out / 'installed-biomcp'
    shutil.copy2(Path(report['candidate_binary']['path']), installed)
    environment = dict(os.environ, BIOMCP_TEST_EXECUTABLE=str(installed))
    installed_report = run(sample, baseline, out / 'installed', True, env=environment)
    assert Path(installed_report['candidate_binary']['path']) == installed
    assert installed_report['candidate_binary']['sha256'] == report['candidate_binary']['sha256']
    malformed = json.loads(sample.read_text())
    pointer = next(iter(malformed['cases'][0]['facts']))
    malformed['cases'][0]['facts'][pointer] = 'summary'
    reject(malformed, baseline, out, 'pointer', 'expected a JSON pointer')
    empty = json.loads(sample.read_text())
    empty['cases'][0].update(case='article/evidence/checked.yaml', step=0, facts={}, assertion_pointers=None)
    reject(empty, baseline, out, 'no-facts', 'selection has no fact assertions')
    print('Recorded parity sample, isolation and assessment-gap checks passed; evidence: ' + str(out))


def reject(plan, baseline, out, name, diagnostic):
    path = out / (name + '.json')
    destination = out / name
    write_json(path, plan)
    result = subprocess.run(['make', 'parity-run', 'PLAN=' + str(path),
                             'BASELINE=' + str(baseline), 'OUT=' + str(destination)],
                            cwd=ROOT, capture_output=True)
    (out / (name + '.log')).write_bytes(result.stdout + result.stderr)
    assert result.returncode != 0 and diagnostic in result.stderr.decode(errors='replace')
    assert not destination.exists(), 'invalid mappings must fail before either binary runs'


if __name__ == '__main__':
    main()

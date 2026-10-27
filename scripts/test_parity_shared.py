"""Prove shared selectors against unchanged family recordings and both binaries.

Arguments: absolute baseline, new evidence directory, Author commit, Residual commit.
Family declarations and their reviewed endpoint catalogs stay owned by those branches.
"""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys

from source_capture import outside_git, write_json
from test_parity_runner import ROOT, run


def declared(commit, path):
    raw = subprocess.run(['git', 'show', commit + ':' + path], cwd=ROOT,
                         capture_output=True, check=True).stdout
    return json.loads(raw)


def witness(commit, batch, case, pointer, wrong, label, baseline, out):
    plan = declared(commit, 'tests/parity/batches/' + batch)
    row = copy.deepcopy(next(row for row in plan['cases'] if row['case'] == case))
    row['assertion_pointers'] = [pointer]
    if label == 'author':
        row['recordings'][0]['request_query'].update(limit='5', offset='0')
    if label == 'limit':
        row['command_selectors'] = {'--limit': {'effective_adverse_event_limit': True}}
    fixtures = out / (label + '-fixtures')
    fixtures.mkdir()
    for name in ('cases', 'replies'):
        (fixtures / name).symlink_to(ROOT / 'tests' / name, target_is_directory=True)
    (fixtures / 'parity').mkdir()
    for name in ('released-sources.json', 'released-endpoints.json'):
        write_json(fixtures / 'parity' / name, declared(commit, 'tests/parity/' + name))
    environment = dict(os.environ, BIOMCP_TEST_FIXTURES=str(fixtures))
    for suffix, projection in [('', row['facts'][pointer]), ('-wrong', wrong)]:
        row['facts'][pointer] = projection
        path = out / (label + suffix + '.json')
        write_json(path, {'baseline_release': '0.9.3', 'cases': [row]})
        report = run(path, baseline, out / (label + suffix), True, env=environment)
        result, = report['cases']
        fact, = result['facts']
        expected = 'wrong' if suffix else 'right'
        assert fact['baseline']['verdict'] == expected, fact
        assert fact['candidate']['verdict'] == 'right', fact
        assert report['assessment_gaps'] == report['regressions'] == 0
        assert result['candidate']['provider_connections'] == 0
        if label == 'author' and not suffix:
            assert fact['baseline']['actual'] == [
                'semantic-scholar:2162869982', 'semantic-scholar:113751091']
        if label == 'limit' and not suffix:
            assert fact['baseline']['actual'] == 5


def main():
    baseline, destination, author, residual = sys.argv[1:]
    baseline, out = Path(baseline), Path(destination)
    assert baseline.is_absolute() and out.is_absolute()
    outside_git(out)
    out.mkdir()
    witness(author, 'author-orcid.json', 'author/search.yaml', '/result/items/*/id',
            {'author_ids': '/providers/0/results/0/identity/id'},
            'author', baseline, out)
    witness(residual, 'residual.json', 'adverse-event/search-policy.yaml',
            '/query/limit', '/pagination/offset', 'limit', baseline, out)
    sys.stdout.write('Author namespace and effective limit recorded witnesses passed: '
                     + str(out) + '\n')


if __name__ == '__main__':
    main()

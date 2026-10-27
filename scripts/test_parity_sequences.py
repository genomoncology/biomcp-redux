"""Run bounded sequence witnesses against both executables and retain raw replies."""
import copy
import hashlib
import json
from pathlib import Path
import subprocess
import sys

from source_capture import outside_git, write_json
from test_parity_runner import ROOT


def select(value, pointer):
    parts = pointer.split('/')[1:] if pointer else []
    def descend(value, tokens):
        if not tokens:
            return value
        token, *rest = tokens
        token = token.replace('~1', '/').replace('~0', '~')
        if token == '*':
            return [descend(item, rest) for item in value]
        return descend(value[int(token)] if isinstance(value, list) else value[token], rest)
    return descend(value, parts)


def raw(output, directory):
    if 'stdout' not in output:
        return None
    for label in ('stdout', 'stderr'):
        data = (directory / output[label]).read_bytes()
        assert len(data) == output[label + '_bytes']
    stream = output.get('json_stream', 'stdout')
    parsed = json.loads((directory / output[stream]).read_bytes())
    assert parsed == output['parsed']
    assert output['elapsed_us'] > 0
    return parsed


def execute(plan, baseline, out):
    result = subprocess.run(['make', 'parity-run', 'PLAN=' + str(plan),
                             'BASELINE=' + str(baseline), 'OUT=' + str(out)],
                            cwd=ROOT, capture_output=True)
    out.with_suffix('.log').write_bytes(result.stdout + result.stderr)
    report = json.loads((out / 'report.json').read_text())
    assert result.returncode != 0 and report['assessment_gaps'] > 0
    assert report['regressions'] == 0
    return report


def check_sequence(row):
    directory = Path(row['evidence_directory'])
    candidate = row['candidate']
    assert 'assessment_error' not in candidate, row
    cli = [raw(exchange, directory) for exchange in candidate['exchanges']]
    actual = candidate['parsed']
    if row['case'] == 'protein/paging.yaml' and row['step'] < 2:
        aggregate = copy.deepcopy(cli[0])
        aggregate['result']['items'] = [item for page in cli for item in page['result']['items']]
        aggregate['next'] = cli[-1]['next']
        assert actual == aggregate
        assert len(cli) > 1
    for fact in row['facts']:
        assert fact['candidate']['verdict'] == 'right', fact
        assert fact['candidate']['actual'] == select(actual, fact['pointer'])
    for fact in row.get('mcp_facts') or []:
        assert fact['candidate']['verdict'] == 'right', fact
    for exchange in candidate['exchanges']:
        if 'mcp' in exchange:
            mcp = exchange['mcp']
            frames = [json.loads(line) for line in (Path(mcp['raw_directory']) / 'mcp.stdout').read_bytes().splitlines()]
            texts = [item['text'] for frame in frames for item in frame.get('result', {}).get('content', []) if item.get('type') == 'text']
            assert any(json.loads(text) == mcp['parsed'] for text in texts)
    old = row['baseline']
    for exchange in old.get('exchanges', []):
        raw(exchange, directory)
    for fact in row['facts']:
        if 'actual' in fact['baseline']:
            assert fact['baseline']['actual'] == select(old['parsed'], fact['baseline_projection'])


def ddinter_inputs(out):
    destination = out / 'ddinter-inputs'
    destination.mkdir()
    expected = set(json.loads((ROOT / 'tests/parity/released-endpoints.json').read_bytes())['fixed_endpoints']['ddinter']['urls'])
    rows = []
    for path in sorted((ROOT / 'tests/replies/drug/ddinter-synthetic/ddinter').glob('*.json')):
        original = path.read_bytes()
        capture = json.loads(original)
        assert capture['source'] == 'ddinter' and capture['url'] in expected
        exchange, = capture['exchanges']
        assert exchange['status'] == 200 and exchange['body']['kind'] == 'text'
        body = exchange['body']['value'].encode()
        filename = capture['url'].rsplit('/', 1)[-1]
        (destination / filename).write_bytes(body)
        (destination / (filename + '.capture.json')).write_bytes(original)
        rows.append({'url': capture['url'], 'filename': filename, 'bytes': len(body),
                     'capture_sha256': hashlib.sha256(original).hexdigest(),
                     'body_sha256': hashlib.sha256(body).hexdigest()})
    assert len(rows) == 8 and {row['url'] for row in rows} == expected
    write_json(out / 'ddinter-inputs.json', {'purpose': 'Retained candidate replay inputs; no released sync receipt', 'files': rows})


def main():
    baseline, out = map(Path, sys.argv[1:])
    outside_git(out)
    out.mkdir()
    ddinter_inputs(out)
    report = execute(ROOT / 'tests/parity/sequences.json', baseline, out / 'sequences')
    for row in report['cases']:
        check_sequence(row)
    cases = {row['case'] for row in report['cases']}
    assert {'trial/walk.yaml', 'schema/errors.yaml', 'drug/ddinter.yaml',
            'study/examples.yaml', 'protein/paging.yaml',
            'pathway/wikipathways-missing.yaml', 'pathway/wikipathways-genes-missing.yaml'} <= cases
    gaps = {row['case'] for row in report['cases'] for fact in row['facts'] if fact['baseline']['verdict'] == 'assessment_gap'}
    assert gaps == {'drug/ddinter.yaml', 'protein/paging.yaml'}, gaps
    order = execute(ROOT / 'tests/parity/exchange-order.json', baseline, out / 'exchange-order')
    row, = order['cases']
    requests = json.loads((out / 'exchange-order/0-baseline-requests.json').read_bytes())
    assert len(requests) >= 2
    for ordinal in range(8):
        filename = '0-source-0.json' if ordinal == 0 else f'0-source-0-exchange-{ordinal}.json'
        source = json.loads((out / 'exchange-order' / filename).read_bytes())
        assert source['exchange'] == ordinal
        assert source['adapted_body'] == source['capture']['exchanges'][ordinal]['body']['value']
    sys.stdout.write(json.dumps({'sequence_rows': len(report['cases']), 'assessment_gap_cases': sorted(gaps), 'ordered_requests': len(requests), 'evidence': str(out)}) + '\n')


if __name__ == '__main__':
    main()

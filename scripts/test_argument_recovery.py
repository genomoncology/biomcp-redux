"""Installed usage recovery through CLI, single MCP and typed MCP."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from release.smoke import environment

CASES = [
    (['get', 'trial', 'NCT07491445', 'summary', 'eligibility', '--source', 'ctgov'],
     'get_trial', {'input': ['NCT07491445'], 'section': ['summary', 'eligibility'], 'source': 'ctgov'}, 'section'),
    (['search', 'diagnostic', '--source', 'fda-cdx', '--gene', 'KRAS'],
     'search_diagnostic', {'source': 'fda-cdx', 'gene': 'KRAS'}, '--gene'),
    (['search', 'pgx', '--gene', 'CYP2D6'],
     'search_pgx', {'gene': 'CYP2D6'}, '--source'),
    (['article', 'text', '39214094'],
     'article_text', {'input': '39214094'}, '--part'),
]


def calls(binary, root, mode):
    frames = [dict(jsonrpc='2.0', id=0, method='initialize', params={
        'protocolVersion': '2025-11-25', 'capabilities': {},
        'clientInfo': {'name': 'argument-recovery', 'version': '1'}}),
        dict(jsonrpc='2.0', method='notifications/initialized')]
    for index, (args, name, fields, _) in enumerate(CASES, 1):
        params = {'name': name, 'arguments': fields} if mode == 'typed' else {
            'name': 'biomcp', 'arguments': {'args': args}}
        frames.append(dict(jsonrpc='2.0', id=index, method='tools/call', params=params))
    result = subprocess.run([str(binary), 'serve', '--offline', '--tool-mode', mode],
                            input=''.join(json.dumps(frame) + '\n' for frame in frames),
                            env=environment(root), cwd=root, capture_output=True, text=True, timeout=60)
    assert result.returncode == 0 and not result.stderr, result.stderr
    replies = {row['id']: row for row in map(json.loads, result.stdout.splitlines())}
    return [replies[index]['result']['structuredContent'] for index in range(1, len(CASES) + 1)]


def main():
    binary = Path(sys.argv[1]).resolve(strict=True)
    failures = []
    report = []
    with tempfile.TemporaryDirectory(prefix='biomcp-argument-recovery-') as scratch:
        root = Path(scratch)
        outputs = []
        for args, _, _, _ in CASES:
            result = subprocess.run([str(binary), *args, '--offline'], env=environment(root),
                                    cwd=root, capture_output=True, text=True, timeout=60)
            assert result.returncode == 2 and not result.stderr, result.stderr
            outputs.append(json.loads(result.stdout))
        for mode, values in [('cli', outputs), ('single', calls(binary, root, 'single')),
                             ('typed', calls(binary, root, 'typed'))]:
            for (args, _, _, option), value in zip(CASES, values):
                report.append(dict(mode=mode, args=args, response=value))
                if value.get('error', {}).get('option') not in {option, '--' + option}:
                    failures.append(f'{mode} {args[:2]}: wrong argument')
                if not isinstance(value.get('error', {}).get('valid_values'), list):
                    failures.append(f'{mode} {args[:2]}: values must be a list')
                corrected = value.get('next', [{}])[0].get('args', [])
                if not corrected or corrected[0] == 'help':
                    failures.append(f'{mode} {args[:2]}: no corrected request')
                    continue
                checked = subprocess.run([str(binary), *corrected, '--offline'],
                                         env=environment(root), cwd=root, capture_output=True,
                                         text=True, timeout=60)
                if json.loads(checked.stdout).get('status') == 'usage':
                    failures.append(f'{mode} {args[:2]}: correction is not parseable')
    stage = Path('target/argument-recovery')
    stage.mkdir(parents=True, exist_ok=True)
    (stage / 'responses.json').write_text(json.dumps(report, indent=2) + '\n')
    for row in report:
        print(json.dumps(row))
    if sys.argv[2] != 'capture' and failures:
        raise SystemExit('\n'.join(failures))
    print(json.dumps({'failures': failures}))


if __name__ == '__main__':
    main()

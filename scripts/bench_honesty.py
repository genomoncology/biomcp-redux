"""Bounded current-source replay probes; retain complete CLI and MCP replies."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from release.smoke import environment


def main():
    binary = Path(sys.argv[1]).resolve()
    output = Path.home() / 'workspace/t' / ('biomcp-0678-' + subprocess.check_output(
        ['git', 'rev-parse', 'HEAD'], text=True).strip())
    output.mkdir(parents=True, exist_ok=True)
    probes = [
        ('trial-search', ['search', 'trial', '--condition', 'melanoma', '--offset', '10'],
         'search_trial', {'condition': ['melanoma'], 'offset': 10}, None),
        ('trial-section', ['get', 'trial', 'NCT01844505', 'results', '--offset', '15'],
         'get_trial', {'input': ['NCT01844505'], 'section': ['results'], 'offset': 15}, None),
        ('faers-or', ['search', 'adverse-event', '--drug', 'imatinib OR nilotinib'],
         'search_adverse_event', {'drug': ['imatinib OR nilotinib']}, None),
        ('resolver', ['get', 'variant', 'CA123643', '--resolver', 'variantvalidator', '--build', 'GRCh38'],
         'get_variant', {'input': 'CA123643', 'resolver': 'variantvalidator', 'build': 'GRCh38'}, 'variant/id-forms'),
        ('population-resolver', ['variant', 'population', '7-117559590-ATCT-A', '--resolver', 'variantvalidator'],
         'variant_population', {'input': '7-117559590-ATCT-A', 'resolver': 'variantvalidator'}, 'variant/id-forms'),
        ('clingen', ['get', 'gene', 'MLH1', 'clingen'],
         'get_gene', {'input': ['MLH1'], 'section': ['clingen']}, 'gene/clingen'),
        ('diagnostic', ['search', 'diagnostic', '--source', 'fda-pma', '--manufacturer', 'Foundation Medicine'],
         'search_diagnostic', {'source': 'fda-pma', 'manufacturer': 'Foundation Medicine'}, 'diagnostic/device-approvals'),
    ]
    with tempfile.TemporaryDirectory(prefix='biomcp-0678-') as temporary:
        for label, args, tool, fields, replay in probes:
            root = Path(temporary) / label
            root.mkdir()
            env = environment(root)
            config = root / 'config/biomcp'
            config.mkdir(parents=True)
            globals_ = ['--replay', str(ROOT / 'tests/replies' / replay)] if replay else ['--offline']
            result = subprocess.run([str(binary), *globals_, *args], env=env,
                                    capture_output=True, timeout=60)
            (output / f'{label}-cli.json').write_bytes(result.stdout)
            (output / f'{label}-cli.stderr').write_bytes(result.stderr)
            for mode in ('single', 'typed'):
                (config / 'config.json').write_text(json.dumps({'mcp_tool_mode': mode}))
                params = {'name': tool, 'arguments': fields} if mode == 'typed' else {
                    'name': 'biomcp', 'arguments': {'args': args}}
                frames = [
                    {'jsonrpc': '2.0', 'id': 0, 'method': 'initialize', 'params': {
                        'protocolVersion': '2024-11-05', 'capabilities': {},
                        'clientInfo': {'name': 'bounded-probe', 'version': '1'}}},
                    {'jsonrpc': '2.0', 'method': 'notifications/initialized'},
                    {'jsonrpc': '2.0', 'id': 1, 'method': 'tools/call', 'params': params},
                ]
                result = subprocess.run([str(binary), *globals_, 'serve'], env=env,
                                        input=''.join(json.dumps(f) + '\n' for f in frames).encode(),
                                        capture_output=True, timeout=60)
                (output / f'{label}-{mode}.jsonl').write_bytes(result.stdout)
                (output / f'{label}-{mode}.stderr').write_bytes(result.stderr)
                rows = [json.loads(line) for line in result.stdout.splitlines()]
                reply = next(row for row in rows if row.get('id') == 1)
                body = reply.get('result', {}).get('structuredContent', reply.get('error'))
                print(json.dumps({'probe': label, 'mode': mode, 'body': body}), flush=True)
    print('Evidence:', output)


if __name__ == '__main__':
    main()

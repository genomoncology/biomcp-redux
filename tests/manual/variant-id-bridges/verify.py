"""Verify retained MCP wire replies; never build, call a source, or start a server."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess


def git(*args):
    return subprocess.check_output(['git', *args], text=True).strip()


def verify(args):
    rows = []
    for path in sorted(args.transcripts.glob('*.json')):
        for execution in json.loads(path.read_text()).get('executions', []):
            frames = [json.loads(word) for word in execution['args']
                      if word.startswith('{')]
            calls = [frame for frame in frames if frame['method'] == 'tools/call']
            assert len(calls) == 1, 'one tool call per retained execution'
            name = calls[0]['params']['name']
            replies = [json.loads(line) for line in bytes(execution['stdout']).splitlines()]
            reply = next(row for row in replies if row.get('id') == calls[0]['id'])
            result = reply['result']
            assert not result['isError'], name
            value = result['structuredContent']
            assert value['query']['input'] == args.input
            assert value['result']['resolution'] == 'resolved'
            candidate, = value['result']['candidates']
            identity = candidate['identity']
            assert args.allele in identity['ids'], name
            form, = [r for r in identity['representations']
                     if r.get('build') == 'GRCh38' and
                     r['expression']['value'] == args.grch38]
            assert form['source'] == 'car', 'source-qualified mapping'
            sections = candidate['sections']
            assert sections['civic']['state'] == 'data', name
            assert sections['population']['state'] == 'data', name
            assert sections['population']['data']['build'] == 'GRCh38'
            profiles = sections['civic']['data']['profiles']
            assert profiles, 'nonempty CIViC rows'
            rows.append({'tool': name, 'status': value['status'],
                         'input': args.input, 'allele': args.allele,
                         'GRCh38': args.grch38, 'civic_profiles': len(profiles),
                         'population_dataset': sections['population']['data']['dataset'],
                         'executable': execution['executable'],
                         'binary_sha256': hashlib.file_digest(
                             Path(execution['executable']).open('rb'), 'sha256').hexdigest(),
                         'retained_wire_file': path.name})
    assert {row['tool'] for row in rows} == {'biomcp', 'get_variant'}
    assert len(rows) == 2, 'both single and typed actual calls'
    same = {}
    for name in ['crates', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml']:
        main = git('rev-parse', args.main + ':' + name)
        compiled = git('rev-parse', args.compiled_source + ':' + name)
        assert main == compiled, name
        same[name] = main
    receipt = {'main': args.main, 'compiled_source': args.compiled_source,
               'identical_product_inputs': same, 'actual_calls': rows,
               'limitation': 'Assumed request from report; original exact evaluation call unavailable. '
               'Original typed case runner rejected text summary; structured response verified here.'}
    args.out.write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for option in ['input', 'allele', 'grch38', 'main', 'compiled-source']:
        parser.add_argument('--' + option, required=True)
    parser.add_argument('--transcripts', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    verify(parser.parse_args())

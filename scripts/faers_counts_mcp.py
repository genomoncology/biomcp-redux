"""Check that single and typed tools map the same FAERS count operation."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

from release.smoke import environment


def main():
    binary = Path(sys.argv[1]).resolve()
    replay = Path('tests/replies/adverse-event/faers-reaction-counts').resolve()
    output = Path('target/faers-counts-mcp')
    output.mkdir(parents=True, exist_ok=True)
    replies = {}
    for mode in ('single', 'typed'):
        with tempfile.TemporaryDirectory(prefix='faers-counts-mcp-') as directory:
            root = Path(directory).resolve()
            config = root / 'config.json'
            config.write_text(json.dumps({'mcp_tool_mode': mode, 'mcp_reply_form': 'both'}))
            tool = 'biomcp' if mode == 'single' else 'search_adverse_event'
            arguments = {'args': ['search', 'adverse-event', 'Synthetic medicine', '--aggregate', 'reactions', '--limit', '2']} if mode == 'single' else {'text': 'Synthetic medicine', 'aggregate': 'reactions', 'limit': 2}
            frames = [
                {'jsonrpc': '2.0', 'id': 1, 'method': 'initialize', 'params': {'protocolVersion': '2025-11-25', 'capabilities': {}, 'clientInfo': {'name': 'faers-counts', 'version': '1'}}},
                {'jsonrpc': '2.0', 'method': 'notifications/initialized', 'params': {}},
                {'jsonrpc': '2.0', 'id': 2, 'method': 'tools/call', 'params': {'name': tool, 'arguments': arguments}},
            ]
            request = ''.join(json.dumps(frame) + '\n' for frame in frames)
            result = subprocess.run([str(binary), 'serve', '--hosted', '--config', str(config), '--replay', str(replay)], input=request, text=True, capture_output=True, env=environment(root), cwd=root, timeout=120)
            (output / f'{mode}-requests.jsonl').write_text(request)
            (output / f'{mode}-replies.jsonl').write_text(result.stdout)
            (output / f'{mode}.stderr').write_text(result.stderr)
            if result.returncode:
                raise RuntimeError(f'{mode} server failed; see retained output')
            answer, = [json.loads(line)['result'] for line in result.stdout.splitlines() if json.loads(line).get('id') == 2]
            body = answer.get('structuredContent', {})
            if answer.get('isError') or body.get('status') != 'complete' or body.get('query', {}).get('aggregate') != 'reactions' or 'reaction_counts' not in body.get('result', {}):
                raise RuntimeError(f'{mode} did not execute reaction aggregation; see retained output')
            replies[mode] = body
    if replies['single'] != replies['typed']:
        raise RuntimeError('single and typed count replies differ')
    (output / 'reply.json').write_text(json.dumps(replies['single'], indent=2) + '\n')
    print(json.dumps({'single': 'pass', 'typed': 'pass', 'reaction_counts': replies['single']['result']['reaction_counts']}))


if __name__ == '__main__':
    main()

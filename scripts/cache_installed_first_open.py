"""Bounded first opens of one verified installed binary; no live SQLite observers."""
import argparse
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import threading
import time


SOURCE = '015aea0a704b7b6176b6988bc6fae707ac0feb38'


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary', type=Path)
    parser.add_argument('binary_sha256')
    parser.add_argument('output', type=Path)
    parser.add_argument('--artifact', required=True, help='Verified installed artifact custody reference')
    parser.add_argument('--container', help='Existing qualified runtime container, with output mounted at its host path')
    args = parser.parse_args()
    binary = args.binary if args.container else args.binary.resolve(strict=True)
    if args.container:
        actual = subprocess.check_output(['docker', 'exec', args.container, 'sha256sum', str(binary)], text=True).split()[0]
    else:
        actual = hashlib.sha256(binary.read_bytes()).hexdigest()
    if actual != args.binary_sha256:
        raise SystemExit('Installed binary digest mismatch; no cases started')
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    write_json(output / 'binding.json', {
        'candidate': SOURCE, 'binary': str(binary), 'binary_sha256': actual,
        'artifact': args.artifact, 'command': ['health', '--offline'],
        'processes_per_round': 8, 'maximum_rounds': 50,
        'timing_limit': ('Docker exec dispatch includes daemon overhead; ' if args.container else 'Native process dispatch duration only; ') + 'successful schema/WAL phase timing is unobservable',
    })
    failed = False
    for round_index in range(50):
        root = output / f'round-{round_index:02}'
        root.mkdir()
        for name in ['home', 'config', 'cache', 'tmp']:
            (root / name).mkdir()
        configuration = root / 'config' / 'biomcp'
        configuration.mkdir()
        write_json(configuration / 'config.json', {'disable_sources': ['open', 'noncommercial', 'restricted']})
        cache = root / 'cache'
        barrier = threading.Barrier(9, timeout=30)
        environments = []
        for index in range(8):
            diagnostic = root / f'{index:02}-sqlite.jsonl'
            diagnostic.write_bytes(b'')
            environments.append({
                'PATH': os.environ.get('PATH', '/usr/bin:/bin'),
                'HOME': str(root / 'home'), 'XDG_CONFIG_HOME': str(root / 'config'),
                'XDG_CACHE_HOME': str(cache), 'TMPDIR': str(root / 'tmp'),
                'BIOMCP_SQLITE_DIAGNOSTICS': str(diagnostic),
            })

        def run(index):
            ready = time.monotonic_ns()
            barrier.wait()
            started = time.monotonic_ns()
            command = [str(binary), 'health', '--offline']
            environment = environments[index]
            if args.container:
                command = ['docker', 'exec']
                for key, value in environment.items():
                    command.extend(['-e', f'{key}={value}'])
                command.extend([args.container, str(binary), 'health', '--offline'])
                environment = None
            with (root / f'{index:02}.stdout').open('wb') as stdout, (root / f'{index:02}.stderr').open('wb') as stderr:
                try:
                    result = subprocess.run(command,
                                            env=environment, stdout=stdout,
                                            stderr=stderr, timeout=30)
                    status = result.returncode
                except subprocess.TimeoutExpired:
                    status = 'watchdog'
            finished = time.monotonic_ns()
            record = {'worker': index, 'ready_ns': ready, 'dispatch_start_ns': started,
                      'dispatch_end_ns': finished, 'duration_ms': (finished - started) / 1e6,
                      'exit': status}
            write_json(root / f'{index:02}-exec.json', record)
            return record

        with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
            futures = [pool.submit(run, index) for index in range(8)]
            release = {'monotonic_ns': time.monotonic_ns(), 'unix_ns': time.time_ns(),
                       'host_load': os.getloadavg(), 'empty_cache': not any(cache.iterdir())}
            barrier.wait()
            records = [future.result() for future in futures]
        write_json(root / 'barrier.json', release)
        diagnostics = []
        local_busy = False
        for index in range(8):
            diagnostics.extend(json.loads(line) for line in (root / f'{index:02}-sqlite.jsonl').read_text().splitlines())
            try:
                envelope = json.loads((root / f'{index:02}.stdout').read_text())
            except (ValueError, UnicodeError):
                envelope = {}
            local_busy |= envelope.get('local_error') == 'busy'
        # All CLI processes have exited. These reads cannot affect first-open locking.
        databases = []
        for path in sorted(cache.rglob('*.sqlite')):
            state = {'path': str(path.relative_to(root))}
            try:
                with sqlite3.connect(path.as_uri() + '?mode=ro', uri=True) as connection:
                    state.update({
                        'journal_mode': connection.execute('PRAGMA journal_mode').fetchone()[0],
                        'owners': connection.execute('SELECT owner,version FROM meta ORDER BY owner').fetchall(),
                        'tables': [row[0] for row in connection.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")],
                    })
            except sqlite3.DatabaseError as error:
                state['post_exit_read_error'] = str(error)
            databases.append(state)
        record = {'round': round_index, 'host_load': release['host_load'],
                  'empty_cache_before_release': release['empty_cache'],
                  'dispatch_spread_ms': (max(row['dispatch_start_ns'] for row in records) - min(row['dispatch_start_ns'] for row in records)) / 1e6,
                  'exits': [row['exit'] for row in records], 'sqlite_errors': diagnostics,
                  'local_busy': local_busy, 'post_exit_databases': databases}
        write_json(root / 'summary.json', record)
        print(json.dumps(record), flush=True)
        if not release['empty_cache'] or not databases or any(row['exit'] != 0 for row in records):
            failed = True
            break
    raise SystemExit(bool(failed))


if __name__ == '__main__':
    main()

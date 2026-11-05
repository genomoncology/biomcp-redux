"""Run changed deadline checks once, or ten times with CPU contention."""
import json
import math
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time


def main():
    root = Path('target/deadline-setup').resolve()
    root.mkdir(parents=True, exist_ok=True)
    with (root / 'build.jsonl').open('w') as output:
        subprocess.run(['cargo', 'build', '--locked', '-p', 'biomcp', '--bin', 'biomcp',
                        '--test', 'cases', '--test', 'tools', '--message-format=json'], stdout=output, check=True)
    binaries = {row['target']['name']: row['executable']
                for line in (root / 'build.jsonl').read_text().splitlines()
                if (row := json.loads(line)).get('reason') == 'compiler-artifact' and row.get('executable')}
    installed = root / 'installed'
    installed.mkdir(exist_ok=True)
    for name, target in [('biomcp', 'biomcp'), ('cases', 'case-runner'), ('tools', 'tool-runner')]:
        shutil.copy2(binaries[name], installed / target)
    env = dict(os.environ, BIOMCP_INSTALLED_BINARY=str(installed / 'biomcp'),
               BIOMCP_CASE_RUNNER=str(installed / 'case-runner'), BIOMCP_INSTALLED_VERSION='1.0.0',
               BIOMCP_EVIDENCE_DIR=str(root / 'fixture-evidence'))
    commands = [
        [sys.executable, '-m', 'unittest', '-v', 'release.test_gate'],
        [sys.executable, 'scripts/typed_harness_preflight.py', 'check'],
        [binaries['cases'], 'gene-sections/gencc-first-publisher', '--test-threads', '2'],
    ]
    loaded = sys.argv[1:] == ['load']
    if loaded:
        commands.append([sys.executable, '-m', 'unittest', '-v', 'release.test_smoke'])
    cores = float(os.environ.get('PM_JOB_CORES', os.cpu_count() or 1))
    count = math.ceil(cores * 1.5) if loaded else 0
    print(json.dumps({'mode': 'load' if loaded else 'focused', 'cores': cores, 'burners': count,
                      'host_load': os.getloadavg(), 'python': sys.version}), flush=True)
    burners = [subprocess.Popen([sys.executable, '-c', 'while True: pass']) for _ in range(count)]
    failures = 0
    try:
        for repeat in range(10 if loaded else 1):
            for index, command in enumerate(commands):
                path = root / f'{repeat:02}-{index:02}.log'
                started = time.monotonic()
                with path.open('wb') as output:
                    result = subprocess.run(command, env=env, stdout=output, stderr=subprocess.STDOUT)
                print(json.dumps({'repeat': repeat, 'command': command, 'exit': result.returncode,
                                  'elapsed_seconds': time.monotonic() - started,
                                  'host_load': os.getloadavg(), 'log': str(path)}), flush=True)
                if result.returncode:
                    print(path.read_text(), flush=True)
                    failures += 1
    finally:
        for child in burners:
            child.terminate()
        for child in burners:
            child.wait()
    print(f'failures: {failures}', flush=True)
    return bool(failures)


if __name__ == '__main__':
    sys.exit(main())

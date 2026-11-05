"""Measure existing successful help-step ceilings with the standard 15% headroom."""
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from release.smoke import environment


def main():
    binary = Path(sys.argv[1]).resolve()
    updated = steps = 0
    with tempfile.TemporaryDirectory(prefix='help-measure-') as scratch:
        env = environment(scratch)
        for path in sorted(Path('tests/cases').rglob('*.yaml')):
            try:
                case = json.loads(path.read_text())
            except json.JSONDecodeError:
                continue
            changed = False
            for step in case.get('steps', []):
                args = step.get('run', [])
                if not args or args[0] != 'help' or step.get('exit', 0) != 0:
                    continue
                if not any(key in step for key in ['max_bytes', 'max_tokens']):
                    continue
                output = subprocess.run([str(binary), '--offline', *args], env=env,
                                        capture_output=True, check=True, timeout=30).stdout
                steps += 1
                for key, actual in [('max_bytes', len(output)), ('max_tokens', math.ceil(len(output) / 4))]:
                    if key in step:
                        measured = math.ceil(actual * 1.15)
                        changed |= step[key] != measured
                        step[key] = measured
            if changed:
                path.write_text(json.dumps(case, indent=2, ensure_ascii=False) + '\n')
                updated += 1
    print(f'help measurements: {steps} existing bounded steps, {updated} case files refreshed')


if __name__ == '__main__':
    main()

"""Run the short landing check, or the full suite with --full, and report stage costs."""
import subprocess
import sys
import time

LANDING = ('pre-job', 'workflow-lint', 'rights', 'audit', 'secrets', 'cases')
FULL = ('pre-job', 'workflow-lint', 'docs-check', 'tensor-proto-check',
        'audit', 'secrets-full', 'library-tests', 'coverage')


def main():
    stages = FULL if sys.argv[1:] == ['--full'] else LANDING
    started = time.monotonic()
    status = 0
    try:
        for stage in stages:
            # Coverage's prerequisites were timed above in this same run.
            options = ['-o', 'docs-check', '-o', 'library-tests'] if stage == 'coverage' else []
            status = subprocess.run([sys.executable, 'scripts/timed_stage.py', stage,
                                     'make', '--no-print-directory', *options, stage]).returncode
            if status:
                return status
        return 0
    finally:
        print(f'gate total_seconds={time.monotonic() - started:.3f} exit={status}', flush=True)


if __name__ == '__main__':
    raise SystemExit(main())

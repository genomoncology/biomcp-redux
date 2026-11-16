"""Run the short landing check, or the full suite with --full, and report stage costs."""
import subprocess
import sys
import time

# Rights, audit and secrets take seconds and need no build, so they stop a bad
# change before the compile in pre-job. The first-read check proves the documented
# examples and first read against the debug binary pre-job builds. Changed-library-tests
# runs the library tests of the crates the branch changed, so a ticket's own tests run (0914).
# Changed-tool-proofs runs the tooling proofs the branch changed, so a broken proof fails the
# gate; the full suite runs them all (0944). Changed-schema-release-check compares the reply
# schemas with the release baseline when the branch changed their inputs; the full suite
# always compares (0997).
LANDING = ('rights', 'audit', 'secrets', 'pre-job', 'first-read-check', 'workflow-lint', 'cases',
           'changed-tool-proofs', 'changed-library-tests', 'changed-schema-release-check')
FULL = ('audit', 'secrets-full', 'pre-job', 'workflow-lint', 'docs-check',
        'tensor-proto-check', 'library-tests', 'schema-release-check', 'tool-proofs', 'coverage')


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

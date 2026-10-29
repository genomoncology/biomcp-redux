"""Exercise the mutation command against native-shaped cargo subprocess replies."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parents[1]
CARGO = r'''import json, os, pathlib, re, sys, time
args = sys.argv[1:]
with open('cargo-calls.jsonl', 'a') as stream:
    stream.write(json.dumps(dict(args=args, build=os.environ.get('CARGO_BUILD_JOBS'),
                                tests=os.environ.get('RUST_TEST_THREADS'))) + '\n')
if '--version' in args:
    print('cargo-mutants ' + os.environ.get('FAKE_VERSION', '27.1.0'))
    sys.exit(0)
if args[0] == 'install':
    sys.exit(0)
names = json.loads(pathlib.Path('native-inventory.json').read_text())
if '--list' in args:
    print(json.dumps(names))
    sys.exit(0)
selected = [row for row in names if re.search(args[args.index('--re') + 1], row['name'])]
output = pathlib.Path(args[args.index('--output') + 1]) / 'mutants.out'
output.mkdir()
phase = lambda duration: dict(phase='Test', duration=duration, process_status='Success',
                             argv=['cargo', 'test', '--package=biomcp', '--test=cases'])
baseline = dict(scenario='Baseline', summary='Success', phase_results=[phase(40)])
mode = os.environ.get('FAKE_MODE', 'mixed')
if mode == 'baseline-failure':
    baseline['summary'] = 'Failure'
    outcomes = [baseline]
else:
    outcomes = [baseline]
    summaries = ['CaughtMutant', 'MissedMutant', 'Unviable', 'Timeout']
    if mode not in ('interrupted', 'omitted'):
        summaries.extend(['CaughtMutant'] * (len(selected) - len(summaries)))
    for mutant, summary in zip(selected, summaries):
        outcomes.append(dict(scenario=dict(Mutant=mutant), summary=summary,
                             phase_results=[phase(2)]))
(output / 'outcomes.json').write_text(json.dumps(dict(outcomes=outcomes)))
if mode == 'interrupted':
    time.sleep(60)
sys.exit(4 if mode == 'baseline-failure' else 2)
'''


@unittest.skipIf(os.name != 'posix', 'native mutation jobs use POSIX process groups')
class MutationCommands(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        release = self.root / 'release'
        release.mkdir()
        for name in ('mutations.py', 'toolchain.py', 'toolchain.json'):
            shutil.copyfile(ROOT / 'release' / name, release / name)
        binary = self.root / 'bin'
        binary.mkdir()
        cargo = binary / 'cargo'
        cargo.write_text('#!' + sys.executable + '\n' + CARGO)
        cargo.chmod(0o755)
        self.env = dict(os.environ, HOME=str(self.root), XDG_CONFIG_HOME=str(self.root),
                        PATH=str(binary) + os.pathsep + os.environ['PATH'],
                        GIT_CONFIG_NOSYSTEM='1')
        self.env.pop('FAKE_MODE', None)
        self.env.pop('FAKE_VERSION', None)
        self.script = str(release / 'mutations.py')
        self.output = self.root / 'result'
        self.plan = self.output / 'plan.json'
        self.source('crates/biomcp-query/src/status.rs', 'fn status() {}\n')
        self.source('crates/biomcp-adapters/src/mapped.rs', 'fn mapped() {}\n')
        self.git('init', '--quiet')
        self.commit('initial')
        self.previous = self.git('rev-parse', 'HEAD').stdout.strip()
        self.source('crates/biomcp-adapters/src/mapped.rs', 'fn mapped() { return; }\n')
        self.commit('change product')
        # Native output is deliberately rule-first. The changed file must take priority.
        inventory = []
        for file in ('crates/biomcp-query/src/status.rs', 'crates/biomcp-adapters/src/mapped.rs'):
            for line in range(12):
                inventory.append(dict(name=f'{file}:{line + 1}:5: replace x -> bool with false',
                                      file=file, package='biomcp-query', genre='FnValue'))
        (self.root / 'native-inventory.json').write_text(json.dumps(inventory))

    def source(self, name, value):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(value)

    def git(self, *args):
        return subprocess.run(['git', *args], cwd=self.root, env=self.env,
                              capture_output=True, text=True, check=True)

    def commit(self, subject):
        self.git('add', '.')
        self.git('-c', 'user.name=Test', '-c', 'user.email=test@example.test',
                 'commit', '--quiet', '-m', subject)

    def call(self, *args, ok=True):
        result = subprocess.run([sys.executable, self.script, *map(str, args)], cwd=self.root,
                                env=self.env, capture_output=True, text=True, timeout=15)
        if ok:
            self.assertEqual(result.returncode, 0, result.stderr)
        else:
            self.assertNotEqual(result.returncode, 0, result.stdout)
        return result

    def prepare(self, *options, started=None):
        self.call('plan', '--previous', self.previous, '--output', self.output,
                  '--started', started if started is not None else time.time(), *options)
        return json.loads(self.plan.read_text())

    def report(self):
        return json.loads((self.output / 'mutation-report.json').read_text())

    def test_install_uses_the_manifest_pin_in_the_owned_job(self):
        self.call('install', '--output', self.root / 'setup', '--started', time.time())
        calls = [json.loads(line) for line in (self.root / 'cargo-calls.jsonl').read_text().splitlines()]
        pin = json.loads((ROOT / 'release/toolchain.json').read_text())['cargo-mutants']
        self.assertEqual(calls[0]['args'], ['install', 'cargo-mutants', '--locked', '--version', pin])

    def test_survivors_do_not_fail_and_scope_preserves_all_unrun_mutants(self):
        value = self.prepare('--measure')
        self.assertEqual(len(value['selected']), 20)
        self.assertIn('mapped.rs', value['selected'][0])
        self.call('shard', '--plan', self.plan, '--index', 0)
        self.call('collect', '--plan', self.plan)
        report = self.report()
        self.assertEqual([len(report[k]) for k in ('caught', 'survivors', 'unviable', 'timeouts', 'unrun')],
                         [17, 1, 1, 1, 4])
        self.assertEqual(report['completed'], 19)
        self.assertFalse(report['calibration_complete'])
        self.assertEqual(report['baseline_seconds'], 40)
        self.assertEqual(report['candidate'], self.git('rev-parse', 'HEAD').stdout.strip())
        self.assertEqual(set(report['unrun']), set(value['inventory']) - set(
            report['caught'] + report['survivors'] + report['unviable'] + report['timeouts']))
        calls = [json.loads(line) for line in (self.root / 'cargo-calls.jsonl').read_text().splitlines()]
        run = next(row for row in calls if '--shard' in row['args'])
        for argument in ('--test=cases', '--locked'):
            self.assertIn('--cargo-arg=' + argument, run['args'])
        self.assertEqual(run['args'][run['args'].index('--test-package') + 1], 'biomcp')
        self.assertEqual(run['args'][run['args'].index('--baseline') + 1], 'run')
        self.assertEqual((run['build'], run['tests']), ('2', '2'))

    def test_native_exit_cannot_hide_missing_selected_outcomes(self):
        self.prepare('--measure')
        self.env['FAKE_MODE'] = 'omitted'
        self.call('shard', '--plan', self.plan, '--index', 0)
        self.call('collect', '--plan', self.plan, ok=False)
        self.assertIn('omitted selected', ' '.join(self.report()['errors']))

    def test_failed_baseline_and_missing_shards_are_errors(self):
        self.prepare('--measure', '--shards', 2)
        self.env['FAKE_MODE'] = 'baseline-failure'
        self.call('shard', '--plan', self.plan, '--index', 0, ok=False)
        self.call('collect', '--plan', self.plan, ok=False)
        report = self.report()
        self.assertEqual(report['completed'], 0)
        self.assertEqual(len(report['unrun']), 24)
        self.assertIsNone(report['seconds_per_viable_mutant'])
        self.assertTrue(any('baseline' in error for error in report['errors']))
        self.assertTrue(any('Shard 1' in error for error in report['errors']))

    def test_deadline_preserves_partial_native_outcomes_without_fresh_budget(self):
        self.prepare('--measure', started=time.time() - 90 * 60 + 3)
        self.env['FAKE_MODE'] = 'interrupted'
        started = time.monotonic()
        self.call('shard', '--plan', self.plan, '--index', 0)
        self.assertLess(time.monotonic() - started, 8)
        self.call('collect', '--plan', self.plan)
        report = self.report()
        self.assertEqual(report['completed'], 3)
        self.assertEqual(len(report['unrun']), 20)
        command = json.loads((self.output / 'shard-0/command.json').read_text())
        self.assertIsNone(command['exit'])

    def test_candidate_scope_reserves_setup_and_baseline_cost(self):
        value = self.prepare('--cost-seconds', 100, '--baseline-seconds', 4000,
                             started=time.time() - 600)
        self.assertIn(len(value['selected']), (7, 8))
        self.assertLessEqual(len(value['selected']) * 100 + 4000 + 600, 90 * 60)

    def test_tool_drift_and_changed_source_reject_execution(self):
        self.prepare('--measure')
        self.env['FAKE_VERSION'] = '0.0.0'
        result = self.call('shard', '--plan', self.plan, '--index', 0, ok=False)
        self.assertIn('pinned', result.stderr)
        self.env.pop('FAKE_VERSION')
        self.source('crates/biomcp-query/src/status.rs', 'fn broken() {}\n')
        result = self.call('shard', '--plan', self.plan, '--index', 0, ok=False)
        self.assertIn('source differs', result.stderr)


if __name__ == '__main__':
    unittest.main()

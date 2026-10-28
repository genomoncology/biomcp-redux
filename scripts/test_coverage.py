"""Check collection isolation through the coverage command with a fake Cargo tool."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CARGO = r'''#!/usr/bin/env python3
import json, os
from pathlib import Path
import sys
args = sys.argv[1:]
profiles = Path(os.environ['PROOF_PROFILES'])
if args[:2] == ['llvm-cov', 'show-env']:
    print('export LLVM_PROFILE_FILE="' + str(profiles / 'default-%m-%p.profraw') + '"')
elif args[:2] == ['llvm-cov', 'clean']:
    profiles.mkdir()
    (profiles / 'unrelated.keep').write_text('keep')
    for index in range(64):
        (profiles / f'stale-{index}.profraw').write_text('stale')
elif args[:2] == ['llvm-cov', 'report']:
    names = [p.name for p in profiles.glob('*.profraw')]
    missing = 100
    if any('cases-' in n for n in names): missing = 40
    if any('contracts-' in n for n in names): missing -= 35
    if any('library-' in n for n in names): missing = 0
    print('TOTAL 100 0 100% 100 0 100% 100', missing, str(100-missing)+'%')
    if '--fail-under-lines' in args:
        raise SystemExit(int(100-missing < float(args[args.index('--fail-under-lines')+1])))
elif args[0] == 'metadata':
    print(json.dumps({'packages':[{'name':'biomcp-types', 'targets':[
        {'name':'drug_contracts','kind':['test']},
        {'name':'unlisted_contract','kind':['test']}]},
        {'name':'biomcp-fetch', 'targets':[{'name':'tensors','kind':['test']}]}]}))
elif args[0] == 'test' and '--no-run' not in args:
    with Path(os.environ['PROOF_CALLS']).open('a') as calls:
        calls.write(' '.join(args) + '\n')
    profile = os.environ['LLVM_PROFILE_FILE'].replace('%m','fake').replace('%p','1')
    Path(profile).write_text('profile')
    if 'library-' in profile:
        for index in range(64):
            Path(profile + f'-{index}.profraw').write_text('profile')
'''


def main():
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        for name in ("scripts/coverage.sh", "scripts/contract_targets.py", "coverage.floor", "sdlc/features.md"):
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        shutil.copytree(ROOT / "crates/biomcp-types", root / "crates/biomcp-types")
        shutil.copytree(ROOT / "crates/biomcp-fetch", root / "crates/biomcp-fetch")
        shutil.copytree(ROOT / "crates/biomcp-adapters", root / "crates/biomcp-adapters")
        subprocess.run(["git", "init", "-q", str(root)], check=True)
        tools = root / "fake-tools"
        tools.mkdir()
        cargo = tools / "cargo"
        cargo.write_text(CARGO)
        cargo.chmod(0o755)
        # Reproduce the operating system's argument ceiling with a small bound.
        remove = tools / "rm"
        remove.write_text("#!/usr/bin/env python3\nimport subprocess, sys\n"
                          "if len(sys.argv) > 32: raise SystemExit('Argument list too long')\n"
                          "raise SystemExit(subprocess.call(['/bin/rm', *sys.argv[1:]]))\n")
        remove.chmod(0o755)
        env = dict(os.environ, PATH=str(tools) + os.pathsep + os.environ["PATH"],
                   PROOF_PROFILES=str(root / "profiles"), PROOF_CALLS=str(root / "calls"),
                   RUST_TEST_THREADS="3")
        result = subprocess.run(["sh", "scripts/coverage.sh"], cwd=root, env=env,
                                capture_output=True, text=True, check=False)
        assert result.returncode == 0, result.stdout + result.stderr
        output = result.stdout
        assert "command cases only; historical comparison; product source): TOTAL 100 0 100% 100 0 100% 100 40 60%" in output, output
        assert "command cases plus reviewed public contracts; product source): TOTAL 100 0 100% 100 0 100% 100 5 95%" in output, output
        assert "unlisted contracts and library tests; excluded from floor; product source): TOTAL 100 0 100% 100 0 100% 100 0 100%" in output, output
        assert (root / "calls").read_text().count("--test drug_contracts") == 1
        assert "--test unlisted_contract" in (root / "calls").read_text()
        calls = (root / "calls").read_text().splitlines()
        cases_calls = [call for call in calls if "--test cases" in call]
        assert len(cases_calls) == 1 and cases_calls[0].endswith("-- --test-threads 3"), cases_calls
        tensor_calls = [call for call in calls if "--test tensors" in call]
        assert len(tensor_calls) == 1 and "--features" not in tensor_calls[0], tensor_calls
        assert not list((root / "profiles").glob("library-*.profraw"))
        assert not list((root / "profiles").glob("stale-*.profraw"))
        assert (root / "profiles/unrelated.keep").read_text() == "keep"
        report = subprocess.run(["sh", "scripts/coverage.sh", "report"], cwd=root, env=env,
                                capture_output=True, text=True, check=False)
        assert report.returncode == 0 and "100 5 95%" in report.stdout, report
    print("Coverage counts listed caller contracts and excludes library and unlisted profiles")


if __name__ == "__main__":
    main()

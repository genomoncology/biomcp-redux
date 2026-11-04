"""Run installed population cases selected by their actual runner list."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from release.artifacts import cargo
from release.gate import stage_fixtures
from scripts.trials import run_selected


def main():
    output = ROOT / 'target/population-counts-proof'
    output.mkdir(parents=True, exist_ok=True)
    installed = output / 'installed'
    installed.mkdir(exist_ok=True)
    product = Path(cargo('dev')['executable'])
    runner = Path(cargo('dev', tests='cases')['executable'])
    binary = installed / 'biomcp'
    case_runner = installed / 'case-runner'
    shutil.copy2(product, binary)
    shutil.copy2(runner, case_runner)
    stage_fixtures(ROOT, installed / 'fixtures')
    environment = os.environ
    environment['BIOMCP_TEST_EXECUTABLE'] = str(binary)
    environment['BIOMCP_TEST_FIXTURES'] = str(installed / 'fixtures')
    environment['BIOMCP_TEST_PYTHON'] = sys.executable
    environment['BIOMCP_TRANSCRIPT_DIR'] = str(output / 'transcripts')
    listed = subprocess.check_output([str(case_runner), '--list'], cwd=installed, text=True)
    (output / 'runner-list.txt').write_text(listed)
    names = [line.removesuffix(': test') for line in listed.splitlines()
             if line.endswith(': test') and any(part in line for part in (
                 'variant-population/', '/variant/id-forms/', '/variant/help-population.yaml'))]
    if not names:
        raise ValueError('population selection is empty')
    (output / 'selected-cases.json').write_text(json.dumps(names, indent=2) + '\n')
    run_selected([str(case_runner)], installed, names, lambda: None)
    binding = {
        'source_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
        'installed_binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
        'case_runner_sha256': hashlib.sha256(case_runner.read_bytes()).hexdigest(),
        'selected_cases': names,
    }
    (output / 'source-binding.json').write_text(json.dumps(binding, indent=2) + '\n')


if __name__ == '__main__':
    main()

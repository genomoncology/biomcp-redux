"""Run the two authorized remaining NCI calls serially and stop on any failure."""
from datetime import datetime, timezone
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time

from trial_or_proof import totals


def main():
    root = Path(sys.argv[1])
    plan = json.loads(Path(__file__).with_name('trial_or_nci_capture.json').read_text())
    if [request['id'] for request in plan['requests']] != ['nci-second', 'nci-or']:
        raise ValueError('remaining NCI plan must contain exactly the two authorized calls')
    not_before = datetime.fromisoformat('2026-10-28T18:38:27+00:00')
    time.sleep(max(0, (not_before - datetime.now(timezone.utc)).total_seconds()))
    result = {}
    archives = []
    for index, request in enumerate(plan['requests']):
        if index:
            time.sleep(plan['spacing_seconds'])
        selected = dict(plan, requests=[request])
        directory = root.with_name(root.name + '-' + request['id'])
        with tempfile.TemporaryDirectory(prefix='trial-or-plan-') as temporary:
            selected_path = Path(temporary) / 'plan.json'
            selected_path.write_text(json.dumps(selected))
            subprocess.run([
                sys.executable, str(Path(__file__).with_name('source_capture.py')),
                '--plan', str(selected_path), '--out', str(directory),
            ], check=True)
        archive = Path(str(directory) + '.tar.gz')
        archives.append(archive)
        # Bounded decoding, positive count and native disease witness before next call.
        checked = totals(archive)
        if request['id'] not in checked:
            raise ValueError('NCI request failed; no further source calls allowed')
        result.update(checked)
    subprocess.run([
        sys.executable, str(Path(__file__).with_name('trial_or_proof.py')),
        '/tmp/biomcp-0414-native.tar.gz', '/tmp/biomcp-0414-nci.tar.gz',
        *map(str, archives),
    ], check=True)


if __name__ == '__main__':
    main()

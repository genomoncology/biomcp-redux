"""Check all retained parity receipts, including later rows and stderr JSON."""
import copy
import json
from pathlib import Path
import sys

from test_parity_runner import verify_outputs


def main():
    evidence = Path(sys.argv[1])
    report = json.loads((evidence / 'report.json').read_text())
    assert len(report['cases']) > 1
    verify_outputs(report, evidence)
    wrong = copy.deepcopy(report)
    wrong['cases'][-1]['baseline']['parsed'] = {'deliberately_wrong': True}
    try:
        verify_outputs(wrong, evidence)
    except AssertionError:
        pass
    else:
        raise AssertionError('a wrong later receipt escaped validation')
    stderr_json = copy.deepcopy(report)
    for row in stderr_json['cases']:
        for label in ('baseline', 'candidate'):
            command = row[label]
            command['stdout'], command['stderr'] = command['stderr'], command['stdout']
            command['stdout_bytes'], command['stderr_bytes'] = (
                command['stderr_bytes'], command['stdout_bytes'])
            command['json_stream'] = 'stderr'
    verify_outputs(stderr_json, evidence)
    print('Every retained receipt, later-row rejection and stderr JSON passed:', evidence)


if __name__ == '__main__':
    main()

"""Compare the recorded enabled selector with retained 0593 evidence."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

root = Path('target/0595-proofs')
baseline = root / 'baseline'
current = root / 'replies'
execution = '8d47fe59d82edb16170bb197ea04559cafa652de96501f67b3a06ee9c5d26b1e.json'
requests = 'fa11658f8ff76f669eaefa45a417f94447d3702b709d315de1d024b50363137b.json'
old = json.loads((baseline / execution).read_text())['executions'][0]
new = json.loads((current / execution).read_text())['executions'][0]
assert bytes(new['stdout']) == bytes(old['stdout']), 'enabled reply changed'
assert json.loads((baseline / requests).read_text()) == json.loads((current / requests).read_text()), 'enabled request changed'
binary = root / 'installed/bin/biomcp'
report = {
    'source_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
    'enabled_reply_sha256': hashlib.sha256(bytes(new['stdout'])).hexdigest(),
    'enabled_request_parity': True,
    'enabled_reply_parity': True,
    'baseline_source': '0593 exact-current, publication-source-bindings.json -> 20bc8a9a3cf4960814ec2f8d27d3564bea838428',
    'files': {},
}
for name in sys.argv[1:]:
    report['files'][name] = hashlib.sha256(Path(name).read_bytes()).hexdigest()
(root / 'source-bindings.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report, indent=2))

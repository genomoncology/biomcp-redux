#!/usr/bin/env python3
"""Synthetic pipe checks only; never starts BioMCP, a client, or a provider."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile

from typed_harness_capture import exchange
from typed_harness_smoke import check_sources


def server():
    for line in sys.stdin.buffer:
        frame = json.loads(line)
        # Whitespace is intentional: forwarding must retain the original bytes.
        response = b'{ "jsonrpc": "2.0", "id": ' + json.dumps(frame["id"]).encode() + b', "result": {"synthetic":true} }\n'
        sys.stdout.buffer.write(response)
        sys.stdout.buffer.flush()


def bridge(path):
    child = subprocess.Popen([sys.executable, __file__, "server"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    try:
        exchange(child, Path(path))
        child.wait(timeout=3)
    finally:
        if child.poll() is None:
            child.kill()
            child.wait()


def request(index, value):
    return json.dumps({"jsonrpc": "2.0", "id": index, "method": "tools/call", "params": {"name": "get_trial", "arguments": {"input": [value]}}}).encode() + b"\n"


def check():
    check_sources()
    with tempfile.TemporaryDirectory(prefix="typed-harness-preflight-") as temporary:
        root = Path(temporary)
        inputs = request(1, "NCT02563002") + request(2, "lung cancer")
        successful = root / "successful"
        successful.mkdir()
        run = subprocess.run([sys.executable, __file__, "bridge", str(successful)], input=inputs, capture_output=True, timeout=5)
        if run.returncode or (successful / "requests.jsonl").read_bytes() != inputs:
            raise RuntimeError(f"synthetic forwarding failed: {run.stderr!r}")
        if (successful / "replies.jsonl").read_bytes() != run.stdout or run.stdout.count(b'"synthetic":true') != 2:
            raise RuntimeError("reply bytes changed or replies were lost")
        refused = root / "refused"
        refused.mkdir()
        run = subprocess.run([sys.executable, __file__, "bridge", str(refused)], input=inputs + request(3, "lung cancer"), capture_output=True, timeout=5)
        if run.returncode == 0 or b"unexpected tools/call" not in run.stderr:
            raise RuntimeError("third-call bound failed")
    print("Synthetic byte forwarding and third-call refusal pass; no product/client/provider ran")


if __name__ == "__main__":
    action = sys.argv[1] if len(sys.argv) > 1 else "check"
    if action == "server":
        server()
    elif action == "bridge":
        bridge(sys.argv[2])
    elif action == "check":
        check()
    else:
        raise SystemExit("expected check, bridge, or server")

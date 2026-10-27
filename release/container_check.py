#!/usr/bin/env python3
"""Exercise the runtime container, including installed process and MCP proofs."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

from smoke import mcp_probe


def main():
    image, arch, proof, version, *extra_args = sys.argv[1:]
    common = ["docker", "run", "--rm", "--memory", "2g", "--cpus", "2", "--platform", "linux/" + arch] + extra_args
    checksum = subprocess.check_output(common + ["--entrypoint", "/usr/bin/sha256sum", image, "/usr/local/bin/biomcp"], text=True).split()[0]
    assert checksum == hashlib.sha256((Path(proof) / "biomcp").read_bytes()).hexdigest(), "container binary differs from the candidate"
    for args, key in [(["schema", "biodata.problem/1"], "$id"), (["help"], "commands")]:
        result = subprocess.run(common + [image] + args, capture_output=True, text=True, check=True)
        value = json.loads(result.stdout)
        assert not result.stderr and (value["result"][key] if key == "commands" else value[key])
    with tempfile.TemporaryDirectory() as root:
        mcp_probe(common + ["-i", image, "serve"], os.environ.copy(), Path(root), version)
    executable = common + ["-v", str(Path(proof).resolve()) + ":/proof:ro", "-e", "BIOMCP_TEST_EXECUTABLE=/usr/local/bin/biomcp", "-e", "BIOMCP_TEST_FIXTURES=/proof/fixtures", "--entrypoint", "/proof/case-runner", image]
    subprocess.run(executable + ["--cases", "/proof/fixtures/cases", "--test-threads", "1"], check=True)
    subprocess.run(executable + ["--installed-process-proof"], check=True)
    subprocess.run(executable + ["--installed-http-proof"], check=True)
    print(json.dumps({"artifact": "docker", "platform": "linux/" + arch, "result": "pass", "checks": ["help", "schemas", "version", "mcp-all-stdout", "replay-cli-mcp", "ordinary-cache-two-process", "mcp-http"]}))


if __name__ == "__main__":
    main()

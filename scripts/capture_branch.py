"""Regenerate a branch's schemas and reference docs and record live replies, on a job machine.

python3 scripts/capture_branch.py '["get","drug","NDA216340","label"]' ...
Builds the CLI, runs make schema-generate and docs-generate, then runs each JSON argument list
with --record into its own folder (named by its position). The checkout's diff and the
recordings go to stdout as one base64 tar.gz between BEGIN and END lines, so the branch that
asked can unpack them: base64 -d | tar xz. Each reply also prints, shortened, for the log.
"""
import base64
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    commands = [json.loads(arg) for arg in sys.argv[1:]]
    subprocess.run(["cargo", "build", "--locked", "-q", "-p", "biomcp", "--bin", "biomcp"],
                   cwd=ROOT, check=True)
    for goal in ("schema-generate", "docs-generate"):
        subprocess.run(["make", "--no-print-directory", goal], cwd=ROOT, check=True)
    binary = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")) / "debug" / "biomcp"
    scratch = Path(tempfile.mkdtemp(prefix="capture-branch-"))
    env = dict(os.environ, XDG_CACHE_HOME=str(scratch / "cache"))
    for at, args in enumerate(commands):
        record = scratch / "record" / str(at)
        record.mkdir(parents=True)
        result = subprocess.run([str(binary), "--record", str(record), *args],
                                capture_output=True, env=env, timeout=600)
        print(f"### {at} exit={result.returncode} biomcp {' '.join(args)}")
        print(result.stdout.decode("utf-8", "replace")[:3000])
    diff = subprocess.run(["git", "diff", "--binary"], cwd=ROOT, capture_output=True,
                          check=True).stdout
    archive = io.BytesIO()
    with tarfile.open(fileobj=archive, mode="w:gz") as tar:
        info = tarfile.TarInfo("generated.patch")
        info.size = len(diff)
        tar.addfile(info, io.BytesIO(diff))
        if (scratch / "record").is_dir():
            tar.add(scratch / "record", arcname="record")
    print("BEGIN capture")
    print(base64.b64encode(archive.getvalue()).decode())
    print("END capture")


if __name__ == "__main__":
    main()

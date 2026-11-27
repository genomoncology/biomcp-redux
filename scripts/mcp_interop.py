"""Run the MCP interop stage with a verified, job-local Node.js release."""

import hashlib
import platform
import subprocess
import sys
import tempfile
import time
from pathlib import Path

NODE_VERSION = "v24.22.3"
# From https://nodejs.org/dist/v24.22.3/SHASUMS256.txt (.tar.xz archives).
NODE_SHA256 = {
    "darwin-arm64": "1d64136c8a43d648d1b3bad10797a4c995b1ca032d2bde8799de43b37f058fe5",
    "darwin-x64": "4daeec962855c1a6e66cda398a25037319dc1bbd4a0939188b3547afbbfb6490",
    "linux-arm64": "6e51a2ada00f0011cc266f65d98922b403d7ad88f809ca2aa615a5fe1ce6e86c",
    "linux-x64": "4f2b175a6d53a5c617a80dba2676238a0688cef28c6da4eba35c7d844e66fcfb",
}


def download_node(directory):
    machine = platform.machine()
    arch = {"aarch64": "arm64", "x86_64": "x64"}.get(machine, machine)
    target = f"{platform.system().lower()}-{arch}"
    expected = NODE_SHA256.get(target)
    if expected is None:
        raise SystemExit(f"MCP interop has no pinned Node.js archive for {target}")

    release = f"node-{NODE_VERSION}-{target}"
    archive = directory / f"{release}.tar.xz"
    url = f"https://nodejs.org/dist/{NODE_VERSION}/{archive.name}"
    started = time.monotonic()
    subprocess.run(["curl", "-q", "--fail", "--silent", "--show-error", "--output", archive, url], check=True)
    print(f"MCP interop downloaded {archive.name} in {time.monotonic() - started:.3f}s", flush=True)
    with archive.open("rb") as stream:
        actual = hashlib.file_digest(stream, "sha256").hexdigest()
    if actual != expected:
        raise SystemExit(f"MCP interop SHA-256 mismatch for {archive.name}")
    subprocess.run(["tar", "-xf", archive, "-C", directory], check=True)
    return directory / release / "bin"


def main(binary):
    project = Path("tests/interop")
    with tempfile.TemporaryDirectory(prefix="biomcp-interop-node-") as directory:
        node_bin = download_node(Path(directory))
        node = node_bin / "node"
        subprocess.run([node, node_bin / "npm", "--prefix", project, "ci", "--ignore-scripts",
                        "--no-audit", "--no-fund", "--prefer-offline"], check=True)
        subprocess.run(["uv", "sync", "--project", project, "--locked", "--python", sys.executable], check=True)
        subprocess.run(["uv", "run", "--project", project, "--locked", "--offline", "--no-sync",
                        "python", project / "check.py", binary, node], check=True)


if __name__ == "__main__":
    main(sys.argv[1])

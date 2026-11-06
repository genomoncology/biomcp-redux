"""Vendor the locked dependency graph before compiling release artifacts offline."""
import atexit
from functools import lru_cache
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


@lru_cache(maxsize=1)
def stage():
    """Reuse one isolated source set for this release command and its children."""
    temporary = tempfile.TemporaryDirectory(
        prefix="biomcp-release-vendor-", dir=Path(tempfile.gettempdir()).resolve())
    atexit.register(temporary.cleanup)
    folder = Path(temporary.name)
    try:
        result = subprocess.run(
            ["cargo", "vendor", "--locked", str(folder / "sources")],
            cwd=ROOT, check=True, stdout=subprocess.PIPE, text=True)
        home = folder / "cargo-home"
        home.mkdir()
        (home / "config.toml").write_text(result.stdout + "\n[net]\noffline = true\n")
    except BaseException:
        temporary.cleanup()
        raise
    return folder


def environment():
    return dict(os.environ, CARGO_HOME=str(stage() / "cargo-home"), CARGO_NET_OFFLINE="true")


def main():
    command = sys.argv[1:]
    if command[:1] == ["--"]:
        command = command[1:]
    if not command:
        raise SystemExit("usage: vendor.py -- COMMAND [ARGUMENTS...]")
    raise SystemExit(subprocess.run(command, cwd=ROOT, env=environment()).returncode)


if __name__ == "__main__":
    main()

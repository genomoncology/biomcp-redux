"""Exercise documentation contracts through the checker command."""
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def check(root):
    return subprocess.run(
        [sys.executable, str(ROOT / "scripts/check_docs.py"), "--root", str(root)],
        capture_output=True, text=True, check=False,
    )


def main():
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        for name in ("ROADMAP.md", "sdlc/features.md"):
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        for name in ("sdlc/planning/slices", "tests/cases", "crates/biomcp/tests/runner"):
            shutil.copytree(ROOT / name, root / name)
        result = check(root)
        assert result.returncode == 0, result.stderr
        roadmap = root / "ROADMAP.md"
        original = roadmap.read_text()
        roadmap.write_text(original.replace("112 commands, 375 options", "113 commands, 375 options", 1))
        result = check(root)
        assert result.returncode != 0 and "parity table differs" in result.stderr, result
        roadmap.write_text(original)
        case = root / "tests/cases/new-command/proof.yaml"
        case.parent.mkdir()
        case.write_text("story: A new command\n")
        result = check(root)
        assert result.returncode != 0 and "unmapped case" in result.stderr, result
        case.unlink()
        features = root / "sdlc/features.md"
        features.write_text(features.read_text() + "\nmissing: A caller sees a result. Cases: `tests/cases/missing/*.yaml`.\n")
        result = check(root)
        assert result.returncode != 0 and "matches no cases" in result.stderr, result
    print("Documentation contracts reject wrong counts and missing case mappings")


if __name__ == "__main__":
    main()

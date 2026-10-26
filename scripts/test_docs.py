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
        capture_output=True,
        text=True,
        check=False,
    )


def main():
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        for name in (
            "ROADMAP.md",
            "sdlc/features.md",
            "sdlc/planning/upgrade.md",
            "sdlc/planning/parity-0.9.3.json",
        ):
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        for name in (
            "sdlc/planning/slices",
            "tests/cases",
            "crates/biomcp/tests/runner",
            "crates/biomcp-types",
        ):
            shutil.copytree(ROOT / name, root / name)
        result = check(root)
        assert result.returncode == 0, result.stderr
        roadmap = root / "ROADMAP.md"
        original = roadmap.read_text()
        roadmap.write_text(
            original.replace("| Ported or renamed |", "| Invented outcome |", 1)
        )
        result = check(root)
        assert result.returncode != 0 and "parity table differs" in result.stderr, (
            result
        )
        roadmap.write_text(original)
        note = root / "sdlc/planning/upgrade.md"
        original_note = note.read_text()
        note.write_text(
            original_note.replace("`gene definition`", "`gene invented`", 1)
        )
        result = check(root)
        assert result.returncode != 0 and "upgrade commands differ" in result.stderr, (
            result
        )
        note.write_text(original_note)
        slice_path = root / "sdlc/planning/slices/gene.md"
        original_slice = slice_path.read_text()
        slice_path.write_text(
            original_slice.replace(
                "Section 1 replaces this leaf", "Section 1 removes this leaf", 1
            )
        )
        result = check(root)
        assert result.returncode != 0 and "upgrade commands differ" in result.stderr, (
            result
        )
        slice_path.write_text(original_slice)
        for changed, expected in (
            (
                original_slice.replace(
                    "`search gene` | Ported", "`search invented` | Ported", 1
                ),
                "unknown inventory leaf",
            ),
            (
                original_slice.replace(
                    "`--type`, `--chromosome`", "`--invented`, `--chromosome`", 1
                ),
                "unknown option",
            ),
            (
                "\n".join(
                    line
                    for line in original_slice.split("\n")
                    if not line.startswith("| `gene definition` |")
                ),
                "parity table differs",
            ),
        ):
            slice_path.write_text(changed)
            result = check(root)
            assert result.returncode != 0 and expected in result.stderr, result
        slice_path.write_text(original_slice)
        case = root / "tests/cases/new-command/proof.yaml"
        case.parent.mkdir()
        case.write_text("story: A new command\n")
        result = check(root)
        assert result.returncode != 0 and "unmapped case" in result.stderr, result
        case.unlink()
        features = root / "sdlc/features.md"
        original_features = features.read_text()
        features.write_text(
            original_features.replace("/drug_contracts`", "/missing_contract`")
        )
        result = check(root)
        assert result.returncode != 0 and "target does not exist" in result.stderr, (
            result
        )
        features.write_text(
            original_features.replace("features: `schema`", "features: `missing`")
        )
        result = check(root)
        assert result.returncode != 0 and "feature does not exist" in result.stderr, (
            result
        )
        features.write_text(
            original_features
            + "\nmissing: A caller sees a result. Cases: `tests/cases/missing/*.yaml`.\n"
        )
        result = check(root)
        assert result.returncode != 0 and "matches no cases" in result.stderr, result
    print(
        "Documentation contracts reject phantom leaves, unknown options, missing leaf accounting, drift and missing case mappings"
    )


if __name__ == "__main__":
    main()

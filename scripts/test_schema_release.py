"""Exercise the release schema gate through its public command."""
import io
import tarfile
from pathlib import Path, PurePosixPath
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class SchemaRelease(unittest.TestCase):
    def test_missing_and_malformed_baselines_fail_closed(self):
        with tempfile.TemporaryDirectory() as folder:
            baseline = Path(folder) / "baseline.json"
            for content in (None, "{", "{}"):
                with self.subTest(content=content):
                    if content is not None:
                        baseline.write_text(content)
                    result = subprocess.run(
                        ["python3", str(ROOT / "scripts/schema_release.py"),
                         "--baseline", str(baseline)],
                        capture_output=True, text=True,
                    )
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn("schema release gate:", result.stderr)

    def test_public_gate_rejects_incompatible_same_major_and_accepts_real_tree(self):
        with tempfile.TemporaryDirectory(prefix="biomcp-schema-proof-") as folder:
            candidate = Path(folder)
            archive = subprocess.check_output(["git", "archive", "--format=tar", "HEAD"], cwd=ROOT)
            with tarfile.open(fileobj=io.BytesIO(archive), mode="r:") as tree:
                members = tree.getmembers()
                for member in members:
                    name = PurePosixPath(member.name)
                    if (name.is_absolute() or ".." in name.parts or "\\" in member.name
                            or not (member.isfile() or member.isdir())):
                        raise ValueError(f"unsafe source archive member: {member.name}")
                tree.extractall(candidate, members=members)
            # Plant a same-major narrowing in both generated types and snapshots.
            before, after = "^[ACGTNRYSWKMBDHV]$", "^[ACGT]$"
            source = candidate / "crates/biodata/src/variant/sequence.rs"
            self.assertIn(before, source.read_text())
            source.write_text(source.read_text().replace(before, after))
            for schema in (candidate / "crates/biomcp-types/schemas").glob("*.json"):
                schema.write_text(schema.read_text().replace(before, after))
            # Ordinary in-tree generation accepts the jointly changed copies.
            subprocess.run([
                "cargo", "test", "--locked", "-p", "biomcp-types", "--features", "schema",
                "--test", "schemas", "checked_in_schemas_match_types", "--", "--exact",
            ], cwd=candidate, check=True)
            result = subprocess.run(
                ["make", "schema-release-check"], cwd=candidate,
                capture_output=True, text=True,
            )
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("incompatible contract", result.stdout + result.stderr)
        subprocess.run(["make", "schema-release-check"], cwd=ROOT, check=True)


if __name__ == "__main__":
    unittest.main()

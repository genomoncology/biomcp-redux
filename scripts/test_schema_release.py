"""Exercise the release schema gate through its public command."""
import io
import json
import os
import shutil
import sys
import tarfile
from pathlib import Path, PurePosixPath
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class SchemaRelease(unittest.TestCase):
    def test_generation_requires_one_named_passing_test(self):
        name = "checked_in_schemas_match_types"
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            (root / "scripts").mkdir()
            shutil.copyfile(ROOT / "scripts/schema_release.py", root / "scripts/schema_release.py")
            cargo = root / "cargo"
            cargo.write_text(
                f"#!{sys.executable}\n"
                "import json, os, pathlib, sys\n"
                "pathlib.Path('invocation.json').write_text(json.dumps({"
                "'args': sys.argv[1:], 'update': os.environ.get('UPDATE_SCHEMAS')}))\n"
                "print(os.environ['TEST_OUTPUT'])\n"
                "sys.exit(int(os.environ.get('TEST_EXIT', '0')))\n"
            )
            cargo.chmod(0o755)
            passed = "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out"
            for output, code, succeeds in (
                ("running 0 tests\ntest result: ok. 0 passed; 0 failed;", 0, False),
                (f"test other_test ... ok\n{passed}", 0, False),
                (f"test {name} ... ignored\n{passed}", 0, False),
                (f"test {name} ... ok\n{passed}", 1, False),
                (f"test {name} ... ok\n{passed}", 0, True),
            ):
                with self.subTest(output=output, code=code):
                    result = subprocess.run(
                        [sys.executable, str(root / "scripts/schema_release.py"), "--generate"],
                        cwd=root, capture_output=True, text=True,
                        env=dict(os.environ, PATH=str(root) + os.pathsep + os.environ['PATH'],
                                 TEST_OUTPUT=output, TEST_EXIT=str(code)),
                    )
                    self.assertEqual(result.returncode == 0, succeeds, result.stdout + result.stderr)
                    if not succeeds and code == 0:
                        self.assertIn(f"{name} must run and pass exactly once", result.stderr)
            invocation = json.loads((root / "invocation.json").read_text())
            self.assertEqual(invocation, {
                "args": ["test", "--locked", "-p", "biomcp-types", "--features", "schema",
                         "--test", "schemas", name, "--", "--exact"],
                "update": "1",
            })

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

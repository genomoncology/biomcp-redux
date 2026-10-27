"""Exercise the release schema gate through its public command."""
import gzip
import hashlib
import io
import json
import tarfile
from pathlib import Path
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
        baseline = json.loads((ROOT / "release/schema-baseline.json").read_text())
        data = (ROOT / "release" / baseline.get("archive")).read_bytes()
        with tempfile.TemporaryDirectory() as folder:
            output = io.BytesIO()
            with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as source:
                with tarfile.open(fileobj=output, mode="w") as target:
                    for member in source.getmembers():
                        content = source.extractfile(member).read()
                        if member.name == "base.json":
                            schema = json.loads(content)
                            schema.update(type="integer")
                            schema.pop("enum", None)
                            content = json.dumps(schema).encode()
                        member.size = len(content)
                        target.addfile(member, io.BytesIO(content))
            planted = gzip.compress(output.getvalue(), mtime=0)
            baseline.update(sha256=hashlib.sha256(planted).hexdigest())
            manifest = Path(folder) / "baseline.json"
            manifest.write_text(json.dumps(baseline))
            (Path(folder) / baseline.get("archive")).write_bytes(planted)
            result = subprocess.run(
                ["python3", str(ROOT / "scripts/schema_release.py"),
                 "--baseline", str(manifest)], capture_output=True, text=True,
            )
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("base: incompatible contract", result.stdout + result.stderr)
        subprocess.run(["make", "schema-release-check"], cwd=ROOT, check=True)


if __name__ == "__main__":
    unittest.main()

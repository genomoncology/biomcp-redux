"""Outside-in checks on the local artifact producer and installed launcher."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


class Artifacts(unittest.TestCase):
    def test_debug_wheel_is_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            subprocess.run(["python3", "release/artifacts.py", "build", "--profile", "dev", "--output", str(root)], check=True)
            result = subprocess.run(["python3", "release/artifacts.py", "inspect", str(next(root.glob("*.whl")))], capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("release profile required", result.stderr)

    def test_selected_missing_executable_has_no_build_fallback(self):
        runner = os.environ["BIOMCP_CASE_RUNNER"]
        for args in [
            ["--cases", "tests/cases/gene", "kras.yaml", "--exact"],
            ["schema-discovery/rejects-bad-cases", "--exact"],
        ]:
            result = subprocess.run([runner] + args, env={**os.environ, "BIOMCP_TEST_EXECUTABLE": "/does-not-exist/biomcp"}, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("No such file", result.stdout)


if __name__ == "__main__":
    unittest.main()

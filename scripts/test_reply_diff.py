"""Exercise reply comparison through its command line; run outside the routine gate."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("reply_diff.py")


class ReplyDiff(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.base = Path(self.directory.name, "base")
        self.head = Path(self.directory.name, "head")
        self.base.mkdir()
        self.head.mkdir()

    def write(self, directory, key, stdout=b'{"status":"complete"}\n', code=0, **extra):
        execution = dict(args=["help"], surface="cli", exit=code, stdout=list(stdout))
        execution.update(extra)
        path = directory / ("case.json" if key.startswith("case/") else "trial.json")
        path.write_text(json.dumps(dict(key=key, executions=[execution])))

    def compare(self):
        return subprocess.run([sys.executable, str(SCRIPT), str(self.base), str(self.head)],
                              capture_output=True, text=True, check=False)

    def test_reply_and_exit_changes_name_the_case_pointer(self):
        key = "case/schema/help.yaml/step/0"
        self.write(self.base, key)
        for stdout, code, pointer in [
            (b'{"status":"completE"}\n', 0, "/0/stdout/status"),
            (b'{"status":"complete"}\n', 7, "/0/exit"),
            (b'{ "status":"complete"}\n', 0, "/0/stdout/@character:"),
        ]:
            with self.subTest(pointer=pointer):
                self.write(self.head, key, stdout, code)
                result = self.compare()
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertIn(key, result.stdout)
                self.assertIn(pointer, result.stdout)

    def test_trial_output_and_arguments_remain_significant(self):
        key = "trial/housekeeping/version/argument-hash/0"
        self.write(self.base, key, b"original\n")
        for output, args in [(b"changed\n", ["help"]), (b"original\n", ["version"])]:
            self.write(self.head, key, output, args=args)
            result = self.compare()
            self.assertEqual(result.returncode, 1, result.stderr)
            self.assertIn(key, result.stdout)

    def test_build_identity_and_owned_paths_are_masked(self):
        key = "case/version.yaml/step/0"
        for directory, suffix in [(self.base, "base"), (self.head, "head")]:
            root = "/tmp/" + suffix
            output = f"biomcp 1.{suffix}\nCommit: {suffix}\nConfig: {root}/config\n"
            self.write(directory, key, output.encode(), args=["version", "--verbose"],
                       version="1." + suffix, root=root)
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, "")

    def test_uncaptured_trials_are_disclosed(self):
        for directory in [self.base, self.head]:
            self.write(directory, "case/help.yaml/step/0")
            (directory / "lifecycle.json").write_text(json.dumps(
                dict(kind="trial", name="mcp/broken-output", uncaptured=True)))
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("mcp/broken-output", result.stderr)


if __name__ == "__main__":
    unittest.main()

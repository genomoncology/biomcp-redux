"""Selection refuses names before preparation and keeps target and directory."""
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from trials import run_selected


class TrialSelection(unittest.TestCase):
    def test_exact_names_share_discovery_target_and_directory(self):
        with tempfile.TemporaryDirectory() as temporary:
            cwd = Path(temporary)
            target = cwd / "cases"
            target.touch()
            listing = subprocess.CompletedProcess([], 0,
                "mcp-parity/bounded-framing: test\nschema-discovery/case.yaml: test\n", "")
            prepared = []
            with patch("trials.subprocess.run", return_value=listing) as run:
                def prepare():
                    prepared.append(run.call_count)
                run_selected(target, cwd, ["mcp-parity/bounded-framing"], prepare)
                self.assertEqual(prepared, [1])
                self.assertEqual(run.call_args_list[0].args[0], [str(target), "--list"])
                self.assertEqual(run.call_args_list[1].args[0],
                    [str(target), "mcp-parity/bounded-framing", "--exact"])
                self.assertTrue(all(call.kwargs["cwd"] == cwd for call in run.call_args_list))

    def test_unknown_and_empty_names_refuse_before_preparation(self):
        with tempfile.TemporaryDirectory() as temporary:
            cwd = Path(temporary)
            target = cwd / "cases"
            target.touch()
            listing = subprocess.CompletedProcess([], 0, "mcp-parity/bounded-framing: test\n", "")
            for names in [[], ["mcp-parity"], ["schema-discovery/rejects-bad-cases"],
                          ["runner::protocol::bounded-framing"], ["bounded-framing"]]:
                with self.subTest(names=names), patch("trials.subprocess.run", return_value=listing) as run:
                    def prepare():
                        self.fail("invalid selection reached build preparation")
                    with self.assertRaises(ValueError):
                        run_selected(target, cwd, names, prepare)
                    self.assertEqual(run.call_count, 1)


if __name__ == "__main__":
    unittest.main()

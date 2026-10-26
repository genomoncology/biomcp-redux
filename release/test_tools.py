"""Exercise the ordinary gate's desktop tool comparison from source only."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class DesktopTools(unittest.TestCase):
    def check(self, root):
        return subprocess.run([sys.executable, str(ROOT / "release/tools.py"), "--root", str(root)], capture_output=True, text=True, check=False)

    def test_manifest_matches_server_and_rejects_an_added_tool(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name in ("manifest.json", "crates/biomcp/src/mcp/protocol.rs", "crates/biomcp/src/mcp/research.rs"):
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / name, target)
            result = self.check(root)
            self.assertEqual(result.returncode, 0, result.stderr)
            source = root / "crates/biomcp/src/mcp/research.rs"
            source.write_text(source.read_text().replace('[("search",', '[("fourth", "query", "A fake fourth tool", "fake/1"), ("search",', 1))
            result = self.check(root)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("desktop manifest tools differ", result.stderr)
            manifest = root / "manifest.json"
            value = json.loads(manifest.read_text())
            value["tools"].append({"name": "fourth", "description": "A fake fourth tool"})
            manifest.write_text(json.dumps(value))
            result = self.check(root)
            self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == "__main__":
    unittest.main()

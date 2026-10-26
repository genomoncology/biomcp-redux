"""Installed smoke contracts still reject broken candidates under Python optimization."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


class OptimizedSmoke(unittest.TestCase):
    def test_broken_installed_commands_fail_under_optimization(self):
        replies = [
            {"jsonrpc": "2.0", "id": 1, "result": {"serverInfo": {"version": "1"}}},
            {"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "biomcp"}]}},
            {"jsonrpc": "2.0", "id": 3, "result": {}},
        ]
        cases = [("schema", {"$id": "wrong"}, "schema identity"),
                 ("help", {"result": {"commands": []}}, "help commands")]
        for mutation, message in [("version", "MCP version"), ("tools", "MCP tool"),
                                  ("protocol", "MCP protocol"), ("ids", "MCP response IDs")]:
            altered = json.loads(json.dumps(replies))
            if mutation == "version":
                altered[0]["result"]["serverInfo"]["version"] = "wrong"
            elif mutation == "tools":
                altered[1]["result"]["tools"][0]["name"] = "wrong"
            elif mutation == "protocol":
                altered[0]["jsonrpc"] = "wrong"
            else:
                altered.pop()
            cases.append(("mcp", altered, message))
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / "candidate"
            for boundary, reply, message in cases:
                with self.subTest(boundary=boundary, message=message):
                    # A real child executable supplies a planted bad candidate reply.
                    output = "\n".join(json.dumps(row) for row in reply) if boundary == "mcp" else json.dumps(reply)
                    binary.write_text(f"#!{sys.executable}\nimport sys\nprint({output!r})\n")
                    binary.chmod(0o755)
                    call = ("mcp_probe([sys.argv[1]], environment(root), root, '1')" if boundary == "mcp"
                            else "smoke(sys.argv[1], sys.argv[1], root, '1')")
                    if boundary == "help":
                        binary.write_text(f"#!{sys.executable}\nimport sys\nprint('{{\"$id\":\"biodata.problem/1\"}}' if sys.argv[1] == 'schema' else {output!r})\n")
                    script = "from release.smoke import *\nimport sys\nroot = Path(sys.argv[2])\n" + call
                    result = subprocess.run([sys.executable, "-O", "-c", script, str(binary), str(root)],
                                            capture_output=True, text=True, timeout=30)
                    self.assertNotEqual(result.returncode, 0, result.stdout)
                    self.assertIn(message, result.stderr)


if __name__ == "__main__":
    unittest.main()

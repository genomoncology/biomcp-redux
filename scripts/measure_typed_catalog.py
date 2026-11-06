"""Measure compact UTF-8 tools/list definitions from an isolated installed server."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import sys


def encoded(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode()


def main():
    if len(sys.argv) == 1:
        subprocess.run(["cargo", "+1.95.0", "build", "--locked", "-p", "biomcp", "--bin", "biomcp"], check=True)
        binary = Path(os.environ.get("CARGO_TARGET_DIR", "target")) / "debug/biomcp"
    else:
        binary = Path(sys.argv[1])
    binary = binary.resolve()
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    frame = {"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {"_meta": {
        "io.modelcontextprotocol/protocolVersion": "2026-07-28",
        "io.modelcontextprotocol/clientCapabilities": {}}}}
    with tempfile.TemporaryDirectory() as scratch:
        env = {key: value for key, value in os.environ.items()
               if key in {"PATH", "SystemRoot", "WINDIR", "TMPDIR", "TEMP", "TMP"}}
        env.update(HOME=scratch, XDG_CONFIG_HOME=scratch, XDG_CACHE_HOME=scratch, APPDATA=scratch)
        for label, flags in (("single", []), ("typed", ["--mcp-tool-mode", "typed"]),
                             ("hosted-typed", ["--mcp-tool-mode", "typed", "--hosted"])):
            result = subprocess.run([str(binary), "serve", "--offline", *flags],
                                    input=json.dumps(frame) + "\n", env=env,
                                    capture_output=True, text=True, timeout=30, check=True)
            reply = json.loads(result.stdout)
            tools = reply["result"]["tools"]
            print(json.dumps({"mode": label, "revision": revision, "tool_count": len(tools),
                              "definition_bytes": len(encoded(tools)),
                              "tools": [{"name": tool["name"], "bytes": len(encoded(tool)),
                                         "description_bytes": len(tool.get("description", "").encode())}
                                        for tool in tools]}, separators=(",", ":")))


if __name__ == "__main__":
    main()

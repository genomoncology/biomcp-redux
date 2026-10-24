#!/usr/bin/env python3
"""Run one installed executable with copied fixtures and no checkout dependency."""
import json
import os
from pathlib import Path
import subprocess
import tempfile


def environment(root):
    # No host keys, proxies, configuration, warm caches, or build PATH.
    root = Path(root).resolve()
    temporary = root / "tmp"
    temporary.mkdir(exist_ok=True)
    return {"HOME": str(root / "home"), "USERPROFILE": str(root / "home"),
            "XDG_CONFIG_HOME": str(root / "config"), "XDG_CACHE_HOME": str(root / "cache"),
            "APPDATA": str(root / "config"), "LOCALAPPDATA": str(root / "cache"),
            "TMPDIR": str(temporary), "TMP": str(temporary), "TEMP": str(temporary)}


def smoke(binary, runner, fixtures, version, server_args=("serve",)):
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory).resolve()
        env = environment(root)
        binary = Path(binary).resolve()
        runner = Path(runner).resolve()
        fixtures = Path(fixtures).resolve()
        def command(args):
            result = subprocess.run([str(binary)] + args, env=env, cwd=root, capture_output=True, timeout=60)
            if result.returncode or result.stderr:
                raise ValueError("installed command failed: " + result.stdout.decode() + result.stderr.decode())
            return json.loads(result.stdout)
        assert command(["schema", "biodata.problem/1"])["$id"] == "biodata.problem/1"
        assert command(["help"])["result"]["commands"]
        # The first tool call creates the ordinary cache; every emitted line,
        # including shutdown, must be a correlated JSON-RPC response.
        tools = mcp_probe([str(binary)] + list(server_args), env, root, version)
        assert any((root / "cache").rglob("*.sqlite")), "first tool call did not create the cache"
        env.update(BIOMCP_TEST_EXECUTABLE=str(binary), BIOMCP_TEST_FIXTURES=str(fixtures))
        subprocess.run([str(runner), "--cases", str(fixtures / "cases"), "--test-threads", "1"], env=env, cwd=root, check=True, timeout=120)
        # The existing process publication test uses the very same selector.
        subprocess.run([str(runner), "--installed-process-proof"], env=env, cwd=root, check=True, timeout=120)
        subprocess.run([str(runner), "--installed-http-proof"], env=env, cwd=root, check=True, timeout=120)
        return tools

def mcp_probe(command, env, root, version):
    frames = [
        {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "installed-gate", "version": "1"}}},
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        {"jsonrpc": "2.0", "id": 2, "method": "tools/list"},
        {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "biomcp", "arguments": {"args": ["get", "gene", "KRAS", "--offline"]}}},
    ]
    result = subprocess.run(command, input="".join(json.dumps(f) + "\n" for f in frames), env=env, cwd=root, capture_output=True, text=True, timeout=60)
    replies = [json.loads(line) for line in result.stdout.splitlines()]
    assert result.returncode == 0 and not result.stderr, result.stderr
    assert len(replies) == 3 and {r["id"] for r in replies} == {1, 2, 3}, replies
    assert all(r["jsonrpc"] == "2.0" for r in replies)
    indexed = {r["id"]: r for r in replies}
    assert indexed[1]["result"]["serverInfo"]["version"] == version
    assert indexed[2]["result"]["tools"][0]["name"] == "biomcp"

    return [tool["name"] for tool in indexed[2]["result"]["tools"]]

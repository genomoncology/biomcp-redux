"""Exercise official MCP clients against a recorded stdio read and the research skill."""

import asyncio
import json
import subprocess
import sys
import tempfile
import time
from importlib.metadata import version
from pathlib import Path

from mcp import Client, StdioServerParameters

CURRENT = "2026-07-28"
LEGACY = "2025-11-25"
TYPESCRIPT = "@modelcontextprotocol/client"
INSPECTOR = "@modelcontextprotocol/inspector"
HERE = Path(__file__).resolve().parent


async def check_python(case, mode, revision):
    server = StdioServerParameters(**case["server"])
    async with Client(server, mode=mode, read_timeout_seconds=15) as client:
        assert client.protocol_version == revision, client.protocol_version
        call = case["call"]
        listing = await client.list_tools()
        assert any(tool.name == call["name"] for tool in listing.tools), "biomcp tool missing"
        result = await client.call_tool(call["name"], call["arguments"])
        assert not result.is_error, "biomcp reported a tool error"
        text = next(block.text for block in result.content if block.type == "text")
        reply = json.loads(text)
        assert reply["status"] == "complete", reply["status"]
        assert reply["result"]["items"][0]["card"]["nct_id"] == case["record"]
        assert result.structured_content == reply


def check_inspector(node, server):
    command = [
        node, HERE / "node_modules/.bin/mcp-inspector", "--cli",
        "--protocol-era", "modern", "--method", "skills/list", "--verify", "--require-digests",
    ]
    for key, value in server["env"].items():
        command.extend(["-e", f"{key}={value}"])
    command.extend(["--", server["command"], *server["args"]])
    started = time.monotonic()
    result = subprocess.run(command, cwd=server["cwd"], env=server["env"])
    elapsed = time.monotonic() - started
    print(f"MCP Inspector skills verification took {elapsed:.3f}s (exit {result.returncode})", flush=True)
    if result.returncode:
        raise SystemExit(result.returncode)


def check(binary, node, package, sdk_version, mode, revision):
    label = f"{package}@{sdk_version} {mode} {revision}"
    print(f"MCP interop {label}", flush=True)
    with tempfile.TemporaryDirectory(prefix="biomcp-interop-") as directory:
        root = Path(directory)
        record = "NCT02578680"
        replay = HERE.parent / "replies" / "trial" / "get" / record
        case = {
            "server": {
                "command": str(binary),
                "args": ["--replay", str(replay), "--case-cache", str(root / "replay"), "serve"],
                "env": {
                    "HOME": str(root),
                    "XDG_CONFIG_HOME": str(root / "config"),
                    "XDG_CACHE_HOME": str(root / "cache"),
                    "XDG_DATA_HOME": str(root / "data"),
                },
                "cwd": str(root),
            },
            "call": {"name": "biomcp", "arguments": {"args": ["get", "trial", record]}},
            "record": record,
        }
        if package == INSPECTOR:
            check_inspector(node, case["server"])
        elif package == "mcp":
            asyncio.run(check_python(case, mode, revision))
        else:
            case_file = root / "case.json"
            case_file.write_text(json.dumps(case), encoding="utf-8")
            subprocess.run([node, HERE / "check.mjs", package, mode, revision, case_file], check=True)
    print(f"MCP interop passed {label}", flush=True)


def main(binary, node):
    dependencies = json.loads((HERE / "package.json").read_text())["dependencies"]
    check(binary, node, INSPECTOR, dependencies[INSPECTOR], "modern", CURRENT)
    for mode, revision in [("auto", CURRENT), ("legacy", LEGACY)]:
        check(binary, node, TYPESCRIPT, dependencies[TYPESCRIPT], mode, revision)
        check(binary, node, "mcp", version("mcp"), mode, revision)
    for package, pin in dependencies.items():
        if package.startswith("mcp-"):
            revision = package.removeprefix("mcp-")
            mode = "auto" if revision == CURRENT else "legacy"
            check(binary, node, package, pin.rsplit("@", 1)[1], mode, revision)


if __name__ == "__main__":
    main(*(Path(arg).resolve(strict=True) for arg in sys.argv[1:]))

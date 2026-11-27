"""Exercise the official MCP clients against one recorded stdio read."""

import asyncio
import json
import shlex
import subprocess
import sys
import tempfile
from importlib.metadata import version
from pathlib import Path

from mcp import Client, StdioServerParameters

CURRENT = "2026-07-28"
LEGACY = "2025-11-25"
TYPESCRIPT = "@modelcontextprotocol/client"
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


def check(binary, package, sdk_version, mode, revision):
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
        if package == "mcp":
            asyncio.run(check_python(case, mode, revision))
        else:
            case_file = root / "case.json"
            case_file.write_text(json.dumps(case), encoding="utf-8")
            command = shlex.join(["node", "check.mjs", package, mode, revision, str(case_file)])
            subprocess.run(["npx", "--offline", "--call", command], cwd=HERE, check=True)
    print(f"MCP interop passed {label}", flush=True)


def main(binary):
    dependencies = json.loads((HERE / "package.json").read_text())["dependencies"]
    for mode, revision in [("auto", CURRENT), ("legacy", LEGACY)]:
        check(binary, TYPESCRIPT, dependencies[TYPESCRIPT], mode, revision)
        check(binary, "mcp", version("mcp"), mode, revision)
    for package, pin in dependencies.items():
        if package.startswith("mcp-"):
            revision = package.removeprefix("mcp-")
            mode = "auto" if revision == CURRENT else "legacy"
            check(binary, package, pin.rsplit("@", 1)[1], mode, revision)


if __name__ == "__main__":
    main(Path(sys.argv[1]).resolve(strict=True))

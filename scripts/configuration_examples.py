"""Execute the MCP calls in the configuration guide against local recordings."""

import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

DOCUMENT = "docs/configuration.md"
REPLAYS = {
    "gene": "gene/main",
    "variant": "variant/id-forms",
    "disease": "configuration-examples",
    "drug": "drug/approvals-newest",
    "trial": "trial/condition",
    "article": "article/search/live",
    "protein": "configuration-examples",
    "pathway": "pathway/union-totals",
    "phenotype": "configuration-examples",
    "adverse_event": "configuration-examples",
    "diagnostic": "diagnostic/gtr-synthetic",
    "cell_line": "configuration-examples",
    "study": "study/card-section-totals",
}
SYNC = {"diagnostic": "gtr", "study": "cbioportal-studies"}
COMPACT_CALL = re.compile(r'^\d+\. Call `(search_tools|call_tool)` with `(\{.+?\})`', re.M)


def isolated_environment(directory):
    env = {name: os.environ[name] for name in ("PATH", "SystemRoot", "WINDIR", "TMPDIR", "TEMP", "TMP")
           if name in os.environ}
    for name, folder in {"HOME": "home", "USERPROFILE": "home", "APPDATA": "config", "LOCALAPPDATA": "cache",
                         "XDG_CONFIG_HOME": "config", "XDG_CACHE_HOME": "cache", "XDG_DATA_HOME": "data"}.items():
        env[name] = str(directory / folder)
    return env


def call(executable, replay, tool, arguments, compact, sync):
    frames = [
        {"id": 1, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "docs", "version": "1"}}},
        {"method": "notifications/initialized"},
        {"id": 2, "method": "tools/call", "params": {"name": tool, "arguments": arguments}},
    ]
    with tempfile.TemporaryDirectory() as scratch:
        directory = Path(scratch)
        env = isolated_environment(directory)
        globals_ = ["--replay", str(replay), "--case-cache", str(directory / "case-cache")]
        if sync:
            prepared = subprocess.run([executable, "sync", sync, *globals_], cwd=directory, env=env,
                                      capture_output=True, text=True, timeout=60)
            if prepared.returncode or not json.loads(prepared.stdout).get("result"):
                raise ValueError(f"sync {sync} failed: {prepared.stdout or prepared.stderr}")
        options = ["--catalog", "compact"] if compact else []
        process = subprocess.run(
            [executable, "serve", "--tool-mode", "typed", *options, *globals_],
            input="".join(json.dumps({"jsonrpc": "2.0", **frame}) + "\n" for frame in frames),
            cwd=directory, env=env, capture_output=True, text=True, timeout=60,
        )
    messages = map(json.loads, process.stdout.splitlines())
    response = next((message for message in messages if message.get("id") == 2), {})
    result = response.get("result", {})
    if process.returncode or not result:
        raise ValueError(f"MCP call failed: {response.get('error', process.stderr)}")
    reply = result.get("structuredContent")
    if reply is None:
        reply = json.loads(result["content"][0]["text"])
    if result.get("isError"):
        raise ValueError(f"MCP call returned {reply.get('error', reply)}")
    return reply


def check_call(root, executable, where, tool, arguments, compact=False):
    replay_tool = {"call_tool": arguments.get("name"), "search_tools": "trial"}.get(tool, tool)
    try:
        replay = root / "tests/replies" / REPLAYS[replay_tool]
        reply = call(executable, replay, tool, arguments, compact, SYNC.get(replay_tool))
        if tool == "search_tools":
            if not any(item["name"] == "trial" for item in reply.get("tools", [])):
                raise ValueError("discovery did not include trial")
        else:
            if reply.get("status") not in {"complete", "partial"} or not reply.get("result"):
                raise ValueError(f"MCP call returned no result: {reply}")
            action = arguments["arguments"] if tool == "call_tool" else arguments
            if action["action"] == "search" and not reply["result"].get("items"):
                raise ValueError(f"MCP search returned no rows: {reply}")
    except (KeyError, ValueError, subprocess.TimeoutExpired) as error:
        raise ValueError(f"{where}: {tool}: {error}") from error


def check(root, executable, examples, entity_tools):
    examples = [(where, tool, arguments) for where, tool, arguments in examples
                if where.startswith(f"{DOCUMENT}:")]
    if {tool for _, tool, _ in examples} != entity_tools:
        raise ValueError(f"{DOCUMENT}: expected a runnable example for each entity tool")
    executable = executable.resolve()
    for where, tool, arguments in examples:
        check_call(root, executable, where, tool, arguments)
    document = (root / DOCUMENT).read_text()
    compact = list(COMPACT_CALL.finditer(document))
    if [match[1] for match in compact] != ["search_tools", "search_tools", "call_tool"]:
        raise ValueError(f"{DOCUMENT}: expected two compact discovery calls and one compact tool call")
    for match in compact:
        line = document.count("\n", 0, match.start()) + 1
        where = f"{DOCUMENT}:{line}"
        check_call(root, executable, where, match[1], json.loads(match[2]), compact=True)
    print(f"Ran {len(examples) + len(compact)} MCP examples from {DOCUMENT}")

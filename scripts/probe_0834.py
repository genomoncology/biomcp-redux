"""Temporary ticket 0834 probe: replay baseline calls and print each reply's error and next call.

Removed before the ticket lands. Usage: python3 scripts/probe_0834.py [BINARY]
"""
import json
import os
import subprocess
import sys
import tempfile

CLI = [
    "search trial -c|spinal muscular atrophy|-i|apitegromab",
    "search trial -c|MCT8 deficiency",
    "search trial -i tecovirimat --limit 30",
    "search trial -q|E4512 crizotinib",
    "search article -q|E4512 crizotinib adjuvant|--limit 5",
    "search study -q|MYC amplified",
    "search gwas -q|mutation rate",
    "search diagnostic --source fda-cdx -q camizestrant",
    "search pgx -g G6PD",
    "search article -k|MYCN MYCL|--limit 8",
    "search trial STOMP tecovirimat --limit 5",
    "search trial ECLIPSE LUNAR --limit 5",
    "search diagnostic G6PD --source who-ivd --limit 50",
    "search diagnostic --source fda-pma colorectal --limit 30",
    "search diagnostic heparin --source fda-device-recall",
    "search variant 2:219088867",
    "search variant rs116855232",
    "search article thiopurine 2026",
    "search drug lenacapavir",
    "search study msk_chord",
    "search diagnostic --source who-ivd --name G6PD",
    "search diagnostic --source fda-cdx --gene ESR1",
    "search diagnostic --source fda-cdx --drug camizestrant",
    "search diagnostic --type fda-cdx --drug camizestrant",
    "search diagnostic --source who-ivd --marker G6PD",
    "search study --query MSK-CHORD",
    "search diagnostic --source fda-device-recall --query heparin",
    "get drug iberdomide label --section indications",
    "get drug ADZYNMA label --section boxed_warning",
    "get drug ADZYNMA label boxed_warning",
    "get drug camizestrant label indications_and_usage",
    "get drug alectinib label --label_section warnings_and_cautions",
    "get drug primaquine who-pq",
    "get drug apitegromab targets",
    "get drug daraxonrasib civic approvals",
    "get pgx G6PD recommendations",
    "get pgx G6PD guidelines",
    "get pgx primaquine recommendations --source cpic",
    "get pgx --gene G6PD --source cpic recommendations",
    "get variant 2-219088867-C-T gnomad",
    "get variant rs1142345 population",
    "get diagnostic P250029 summary",
    "get gene MSTN --source hgnc",
    "get gene MSTN hgnc",
    "get gwas GCST90994124 loci --section-limit 50",
    "get drug tecovirimat regulatory --region ema",
    "search drug rifapentine --region who",
    "search pgx --gene G6PD --drug primaquine",
    "get pgx recommendations --gene G6PD --source cpic --drug primaquine",
    "search pgx primaquine",
    "search drug primaquine --who-pq --limit 50",
    "diagnostic summary P250029 --source fda-pma",
    "help study mutations",
    "get trial E4512",
    "get trial NCT04975997 eligibility",
    "get article 24996524 text",
    "get article 24996524 --section fulltext",
    "get article 24996524 full_text",
    "get disease OMIM:621585 genes",
    "search article --drug crizotinib --keyword E4512",
]

TYPED = [
    {"name": "article", "arguments": {"action": "get", "get": {"id": "24996524", "section": "text"}}},
    {"name": "gene", "arguments": {"action": "get", "get": {"id": "MSTN", "section": "hgnc"}}},
    {"name": "drug", "arguments": {"action": "get", "get": {"id": "ISEMBYLD", "section": ["targets"]}}},
    {"name": "drug", "arguments": {"action": "get", "get": {"id": "imatinib", "section": ["indications"]}}},
    {"name": "drug", "arguments": {"action": "get", "get": {"id": "primaquine", "section": "who-pq"}}},
    {"name": "variant", "arguments": {"action": "get", "get": {"id": "rs1142345", "section": "gnomad"}}},
    {"name": "trial", "arguments": {"action": "search", "search": {"query": "E4512"}}},
]


def words(line):
    parts = line.split("|")
    out = parts[0].split()
    for part in parts[1:]:
        if part.startswith("-") or " " not in part:
            out.extend(part.split())
        else:
            out.append(part)
    return out


def brief(text):
    try:
        reply = json.loads(text)
    except ValueError:
        return text.strip()[:300]
    error = reply.get("error") or {}
    return json.dumps(
        {
            "status": reply.get("status"),
            "code": error.get("code"),
            "option": error.get("option"),
            "message": error.get("message"),
            "valid": (error.get("valid_values") or [])[:10],
            "next": [step.get("args") or step.get("arguments") for step in reply.get("next", [])][:3],
            "query": reply.get("query") if error == {} else None,
        }
    )


def main():
    binary = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.environ.get("CARGO_TARGET_DIR", "target"), "debug", "biomcp"
    )
    home = tempfile.mkdtemp()
    env = dict(os.environ, HOME=home, XDG_CONFIG_HOME=home + "/config", XDG_CACHE_HOME=home + "/cache")
    for line in CLI:
        args = words(line)
        done = subprocess.run([binary, "--offline", *args], capture_output=True, text=True, env=env)
        print("CLI", json.dumps(args), "->", brief(done.stdout or done.stderr))
    frames = [
        {"jsonrpc": "2.0", "id": 1, "method": "initialize",
         "params": {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "p", "version": "0"}}},
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        {"jsonrpc": "2.0", "id": 2, "method": "tools/list"},
    ]
    for index, call in enumerate(TYPED):
        frames.append({"jsonrpc": "2.0", "id": 10 + index, "method": "tools/call", "params": call})
    done = subprocess.run(
        [binary, "serve", "--offline", "--tool-mode", "typed"],
        input="\n".join(json.dumps(f) for f in frames) + "\n",
        capture_output=True, text=True, env=env, timeout=300,
    )
    for line in done.stdout.splitlines():
        reply = json.loads(line)
        if reply.get("id") == 2:
            tools = reply["result"]["tools"]
            print("TOOLS_LIST bytes=%d" % len(json.dumps(tools, separators=(",", ":"), ensure_ascii=False).encode()))
            for tool in tools:
                if tool["name"] in ("article", "gene"):
                    get = tool["inputSchema"]["properties"].get("get", {})
                    print("TOOL", tool["name"], json.dumps(get)[:1500])
        elif isinstance(reply.get("id"), int) and reply["id"] >= 10:
            result = reply.get("result", {})
            value = result.get("structuredContent")
            if value is None:
                text = result.get("content", [{}])[0].get("text", "")
                value = text[text.find("{"):] if "{" in text else text
            else:
                value = json.dumps(value)
            print("TYPED", json.dumps(TYPED[reply["id"] - 10]["arguments"]), "->", brief(value))


if __name__ == "__main__":
    main()

"""Exercise documentation contracts through the checker command."""

import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

import check_docs
from check_docs import ENTITY_TOOLS, SHARED_TOOLS, mapping
from check_front_map import check_configuration_catalog, compact_bytes
from parity import LEDGER_END, LEDGER_START, UPGRADE_END, UPGRADE_START, table, upgrade

ROOT = Path(__file__).resolve().parents[1]


def check(root):
    return subprocess.run(
        [sys.executable, str(ROOT / "scripts/check_docs.py"), "--root", str(root)],
        capture_output=True,
        text=True,
        check=False,
    )


def replace_block(text, start, end, generated):
    return text.split(start, 1)[0] + generated + text.split(end, 1)[1]


FAKE_SERVER = """#!/usr/bin/env python3
import json, sys
ACTIONS = %s
if sys.argv[1] == "sync":
    print(json.dumps({"status": "complete", "result": {"rows": 1}}))
    sys.exit(0)
tools = []
for name in %s:
    schema = {"type": "object", "required": ["action"], "additionalProperties": False,
              "properties": {"action": {"enum": ACTIONS},
                             "get": {"type": "object", "properties": {"id": {"type": ["array", "string"]}}},
                             "search": {"type": "object"}}}
    tools.append({"name": name, "inputSchema": schema})
for line in sys.stdin:
    message = json.loads(line)
    if message.get("method") == "tools/list":
        result = {"tools": tools}
    elif message.get("method") == "tools/call":
        reply = ({"tools": [{"name": "trial"}]} if message["params"]["name"] == "search_tools"
                 else {"status": "complete", "result": {"items": [{}]}})
        result = {"structuredContent": reply}
    else:
        continue
    print(json.dumps({"jsonrpc": "2.0", "id": message["id"], "result": result}), flush=True)
"""


def tools_check(root, readme, original_readme):
    """A retired tool name, a bad example and a missing example each fail the checker."""
    notes = root / "release/notes.md"
    original_notes = notes.read_text()
    for path, text in ((readme, original_readme), (notes, original_notes)):
        path.write_text(text + "\nCall get_gene for a gene.\n")
        result = check(root)
        assert result.returncode != 0 and "retired tool name get_gene" in result.stderr, result
        path.write_text(text)
    notes.write_text(original_notes.replace("| `search_gene` | `gene` | `search` |\n", "", 1))
    result = check(root)
    assert result.returncode != 0 and "does not list every retired tool name" in result.stderr, result
    notes.write_text(original_notes)
    readme.write_text(original_readme.replace('`{"action":"get","get":{"id":"BRAF"}}`', '`{"action":"get","id":"BRAF"}`'))
    result = check(root)
    assert result.returncode != 0 and "not an action with one object" in result.stderr, result
    readme.write_text(original_readme)
    server = root / "fake-biomcp"
    actions = sorted({row[2] for row in mapping(root)} - {"none"})
    server.write_text(FAKE_SERVER % (repr(actions), repr(sorted(ENTITY_TOOLS | SHARED_TOOLS))))
    server.chmod(0o755)
    run = lambda: subprocess.run([sys.executable, str(ROOT / "scripts/check_docs.py"), "--root", str(root), "--executable", str(server)],
                                 capture_output=True, text=True, check=False)
    result = run()
    assert result.returncode == 0, result.stderr
    readme.write_text(original_readme.replace('`{"action":"get","get":{"id":"BRAF"}}`', '`{"action":"get","get":{"id":5}}`'))
    result = run()
    assert result.returncode != 0 and "fails its schema" in result.stderr, result
    readme.write_text(original_readme.replace('`{"action":"get","get":{"id":"BRAF"}}`', '`{"action":"fetch","fetch":{"id":"BRAF"}}`'))
    result = run()
    assert result.returncode != 0 and "does not offer" in result.stderr, result
    readme.write_text(original_readme)


def commercial_check(root, readme, original_readme):
    """Commercial setup comes from the named configuration section, not other JSON."""
    config = root / "docs/configuration.md"
    original = config.read_text()
    readme.write_text(original_readme + '\n```json\n{"format": "md"}\n```\n')
    config.write_text('```json\n{"format": "md"}\n```\n\n' + original)
    result = check(root)
    assert result.returncode == 0, result.stderr
    readme.write_text(original_readme)
    config.write_text(original.replace('["noncommercial", "licensed"]', '["noncommercial"]', 1))
    result = check(root)
    assert result.returncode != 0 and "commercial setup differs" in result.stderr, result
    for changed in (
        original.replace("## Local source exclusions", "## Local source choices", 1),
        original.replace('```json\n{"disable_sources": ["noncommercial", "licensed"]}\n```', '', 1),
        original.replace('```json\n{"disable_sources": ["noncommercial", "licensed"]}\n```', '```json\n{invalid}\n```', 1),
    ):
        config.write_text(changed)
        result = check(root)
        assert result.returncode != 0 and "local source exclusions" in result.stderr, result
    config.write_text(original)


def typed_catalog_ceiling(root):
    docs = root / "docs"
    docs.mkdir(parents=True)
    (docs / "configuration.md").write_text("## MCP tools\n\n| `trial` | Trial | `search` |\n")
    tool = {"name": "trial", "description": "", "inputSchema": {"properties": {"action": {}, "search": {}}}}
    tools = [tool]
    available = 196_608 - compact_bytes(tools)
    tool["description"] = "x" * (available - 1)
    check_configuration_catalog(root, tools, compact_bytes(tools))
    tool["description"] += "x"
    check_configuration_catalog(root, tools, compact_bytes(tools))
    tool["description"] += "x"
    try:
        check_configuration_catalog(root, tools, compact_bytes(tools))
    except SystemExit as error:
        assert "196,609 bytes, over the 196,608-byte ceiling" in str(error), error
    else:
        raise AssertionError("an oversized typed catalog passed the docs check")


def main():
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        for name in (
            "ROADMAP.md",
            "README.md",
            "skills/SKILL.md",
            "docs/reference/commands.md",
            "docs/configuration.md",
            "release/notes.md",
            "sdlc/features.md",
            "sdlc/planning/upgrade.md",
            "sdlc/planning/architecture.md",
            "sdlc/planning/parity-0.9.3.json",
            "manifest.json",
            "crates/biomcp/src/mcp/protocol.rs",
            "crates/biomcp/src/mcp/research.rs",
        ):
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        for name in (
            "sdlc/planning/slices",
            "tests/cases",
            "crates/biomcp/tests/runner",
            "crates/biomcp-types",
            "crates/biomcp-fetch",
            "crates/biomcp-adapters",
        ):
            shutil.copytree(ROOT / name, root / name)
        architecture = root / "sdlc/planning/architecture.md"
        original_architecture = architecture.read_text()
        for name in re.findall(r"Check: `([^`]+)`", original_architecture):
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        reference = root / "docs/reference/commands.md"
        reference.write_text(reference.read_text().replace("biomcp_help", "help"))  # as 0763's regeneration does
        result = check(root)
        assert result.returncode == 0, result.stderr
        readme = root / "README.md"
        original_readme = readme.read_text()
        readme.write_text(original_readme.replace("get gene KRAS", "get gene INVENTED"))
        result = check(root)
        assert result.returncode != 0 and "README examples differ" in result.stderr, result
        readme.write_text(original_readme)
        architecture.write_text(original_architecture + "\n## Planted rule\n\nCheck: `scripts/missing_check.py`.\n")
        result = check(root)
        assert result.returncode != 0 and "architecture names a missing check file" in result.stderr, result
        architecture.write_text(original_architecture + "\n## Planted rule\n\nA rule without enforcement.\n")
        result = check(root)
        assert result.returncode != 0 and "architecture section lacks a check" in result.stderr, result
        architecture.write_text(original_architecture)
        guide = root / "skills/SKILL.md"
        original_guide = guide.read_bytes()
        guide.write_text(re.sub(r"```sh\n.*?```", "```sh\nbiomcp get trial NCT00000000\n```",
                                original_guide.decode(), flags=re.S))
        result = check(root)
        assert result.returncode != 0 and "agent skill example has no successful replay case" in result.stderr, result
        guide.write_bytes(b"x" * 4_600)
        assert check(root).returncode == 0
        guide.write_bytes(b"x" * 4_601)
        result = check(root)
        assert result.returncode != 0 and "agent skill exceeds 4,600 bytes" in result.stderr, result
        guide.write_bytes(original_guide)
        roadmap = root / "ROADMAP.md"
        original = roadmap.read_text()
        roadmap.write_text(
            original.replace("| Ported or renamed |", "| Invented outcome |", 1)
        )
        result = check(root)
        assert result.returncode != 0 and "parity table differs" in result.stderr, (
            result
        )
        roadmap.write_text(original)
        accounting = root / "sdlc/planning/slices/cross-cutting.md"
        original_accounting = accounting.read_text()
        note = root / "sdlc/planning/upgrade.md"
        original_note = note.read_text()
        for outcome in ("Deferred by ruling", "Dropped by ruling"):
            accounting.write_text(original_accounting.replace(
                "| `health` | Ported or renamed |", f"| `health` | {outcome} |", 1
            ))
            roadmap.write_text(replace_block(original, LEDGER_START, LEDGER_END, table(root)))
            note.write_text(replace_block(original_note, UPGRADE_START, UPGRADE_END, upgrade(root)))
            result = check(root)
            assert result.returncode != 0 and "appears in command reference" in result.stderr, result
        accounting.write_text(original_accounting)
        roadmap.write_text(original)
        note.write_text(original_note)
        note = root / "sdlc/planning/upgrade.md"
        original_note = note.read_text()
        note.write_text(
            original_note.replace("`gene definition`", "`gene invented`", 1)
        )
        result = check(root)
        assert result.returncode != 0 and "upgrade commands differ" in result.stderr, (
            result
        )
        note.write_text(original_note)
        slice_path = root / "sdlc/planning/slices/gene.md"
        original_slice = slice_path.read_text()
        slice_path.write_text(
            original_slice.replace(
                "Section 1 replaces this leaf", "Section 1 removes this leaf", 1
            )
        )
        result = check(root)
        assert result.returncode != 0 and "upgrade commands differ" in result.stderr, (
            result
        )
        slice_path.write_text(original_slice)
        for changed, expected in (
            (
                original_slice.replace(
                    "`search gene` | Ported", "`search invented` | Ported", 1
                ),
                "unknown inventory leaf",
            ),
            (
                original_slice.replace(
                    "| `search gene` | Ported or renamed | `--type`",
                    "| `search gene` | Ported or renamed | `--invented`",
                    1,
                ),
                "unknown option",
            ),
            (
                "\n".join(
                    line
                    for line in original_slice.split("\n")
                    if not line.startswith("| `gene definition` |")
                ),
                "parity table differs",
            ),
        ):
            assert changed != original_slice, expected
            slice_path.write_text(changed)
            result = check(root)
            assert result.returncode != 0 and expected in result.stderr, result
        slice_path.write_text(original_slice)
        features = root / "sdlc/features.md"
        original_features = features.read_text()
        features.write_text(
            original_features
            + "\nmissing: A caller sees a result. Cases: `tests/cases/missing/*.yaml`.\n"
        )
        result = check(root)
        assert result.returncode != 0 and "invalid or duplicate feature line" in result.stderr, result
        features.write_text(original_features)
        folder = "work" + "space"  # split so this file passes the check it tests
        for planted in (f"cd ~/{folder}/biomcp\n", f"cp $HOME/{folder}/t/x .\n"):
            readme.write_text(original_readme + planted)
            result = check(root)
            assert result.returncode != 0 and "README.md:" in result.stderr and "workspace path" in result.stderr, result
        readme.write_text(original_readme)
        record = root / "sdlc/planning/upgrade.md"
        original_record = record.read_text()
        record.write_text(original_record + f"Evidence at ~/{folder}/data/x.\n")
        result = check(root)
        assert result.returncode != 0 and "sdlc/planning/upgrade.md:" in result.stderr, result
        # A made-up machine name joins the hashed set, so this test names no real machine. It is
        # split because the checker also reads this file.
        machine = "planted" + "host"
        check_docs.INTERNAL_WORDS.add(check_docs.digest(machine))
        record.write_text(original_record + f"Ran on {machine}.\n")
        try:
            check_docs.internal_names(root)
            raise AssertionError("a machine name in sdlc/ passed the internal-word check")
        except ValueError as error:
            assert str(error).startswith("sdlc/planning/upgrade.md:"), error
        record.write_text(original_record)
        check_docs.internal_names(root)
        check_docs.INTERNAL_WORDS.discard(check_docs.digest(machine))
        readme.write_text(original_readme + "See https://example.org/home/about and /home/runner/work.\n")
        assert check(root).returncode == 0, check(root).stderr
        readme.write_text(original_readme)
        # An NCIt example without its disease fails; with it, it passes (1007).
        cautions = root / "docs/cautions.md"
        cautions.write_text("NCI needs NCIt codes such as C3224.\n")
        result = check(root)
        assert result.returncode != 0 and "docs/cautions.md:1: NCIt example C3224" in result.stderr, result
        cautions.write_text("Use --condition-id NCIT:C3224 (melanoma), or MedGen:C2676676.\n")
        assert check(root).returncode == 0, check(root).stderr
        cautions.write_text("NCI needs NCIt codes such as C9120 (pancreatic ductal adenocarcinoma).\n")
        result = check(root)
        assert result.returncode != 0 and "NCIt example C9120" in result.stderr, result
        cautions.unlink()
        schema = root / "crates/biomcp-types/schemas/option-message-test.json"
        schema.parent.mkdir(parents=True, exist_ok=True)
        schema.write_text('{\n  "pattern": "^Use NCIT:C3224$"\n}\n')
        result = check(root)
        assert result.returncode != 0 and "option-message-test.json:2: NCIt example C3224" in result.stderr, result
        schema.write_text(json.dumps({"pattern": r"^Use NCIT:C3224 \(melanoma\)$"}, indent=2))
        assert check(root).returncode == 0, check(root).stderr
        schema.unlink()
        commercial_check(root, readme, original_readme)
        tools_check(root, readme, original_readme)
        typed_catalog_ceiling(root / "catalog-check")
        manifest = root / "manifest.json"
        original_manifest = manifest.read_text()
        assert '"name": "fetch"' in original_manifest
        manifest.write_text(original_manifest.replace('"name": "fetch"', '"name": "fetch_record"', 1))
        result = check(root)
        assert result.returncode != 0 and "desktop manifest tools differ" in result.stderr, result
        manifest.write_text(original_manifest)
    print(
        "Documentation contracts reject missing architecture checks, phantom leaves, unknown options, missing leaf accounting, drift, invalid feature declarations, a wrong desktop manifest tool name, internal workspace paths, machine names in sdlc records and NCIt examples without their disease"
    )


if __name__ == "__main__":
    main()

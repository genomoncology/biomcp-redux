"""Check generated parity and feature ownership against the checked-out tree."""
import argparse
import re
import sys
from pathlib import Path

from contract_targets import targets
from parity import LEDGER_END, LEDGER_START, UPGRADE_END, UPGRADE_START, rows, table, upgrade


def check(root):
    roadmap = (root / "ROADMAP.md").read_text()
    if roadmap.count(LEDGER_START) != 1 or roadmap.count(LEDGER_END) != 1:
        raise ValueError("parity table differs: expected one generated ledger block")
    actual = LEDGER_START + roadmap.split(LEDGER_START, 1)[1].split(LEDGER_END, 1)[0] + LEDGER_END
    if actual != table(root):
        raise ValueError("parity table differs: run python3 scripts/parity.py and replace its block")
    note = (root / "sdlc/planning/upgrade.md").read_text()
    if note.count(UPGRADE_START) != 1 or note.count(UPGRADE_END) != 1:
        raise ValueError("upgrade commands differ: expected one generated block")
    actual = UPGRADE_START + note.split(UPGRADE_START, 1)[1].split(UPGRADE_END, 1)[0] + UPGRADE_END
    if actual != upgrade(root):
        raise ValueError("upgrade commands differ: run python3 scripts/parity.py --upgrade and replace its block")
    reference = (root / "docs/reference/commands.md").read_text()
    commands = set(re.findall(r"^## biomcp (.+)$", reference, re.MULTILINE))
    for path, (command, outcome, *_) in rows(root):
        name = command.strip("`")
        # 0.9 placed the bundle before sync; the current grammar places sync first.
        current = "sync " + name.removesuffix(" sync") if name.endswith(" sync") else name
        if outcome in ("Deferred by ruling", "Dropped by ruling") and current in commands:
            raise ValueError(f"{path}: {command} is {outcome.lower()} but appears in command reference")
    cases = {path.relative_to(root).as_posix() for path in (root / "tests/cases").rglob("*.yaml")}
    if not cases:
        raise ValueError("no outside-in cases found")
    mapped = set()
    names = set()
    contracts = {row[3] for row in targets(root)}
    for line in (root / "sdlc/features.md").read_text().splitlines():
        if not line or line.startswith("#"):
            continue
        if "Contracts:" in line:
            name = line.split(":", 1)[0]
            if name in names or name not in contracts:
                raise ValueError(f"invalid or duplicate feature line: {line}")
            names.add(name)
            continue
        match = re.fullmatch(r"([a-z][a-z0-9-]*): (.+[.!?]) Cases: (.+)\.", line)
        if not match or match[1] in names:
            raise ValueError(f"invalid or duplicate feature line: {line}")
        names.add(match[1])
        patterns = re.findall(r"`([^`]+)`", match[3])
        if not patterns or match[3] != ", ".join(f"`{pattern}`" for pattern in patterns):
            raise ValueError(f"invalid case references for {match[1]}")
        for pattern in patterns:
            native = pattern.startswith("crates/biomcp/tests/runner/") and pattern.endswith(".rs")
            if not (pattern.startswith("tests/cases/") or native) or ".." in Path(pattern).parts:
                raise ValueError(f"case reference escapes tests/cases: {pattern}")
            found = {path.relative_to(root).as_posix() for path in root.glob(pattern) if path.is_file()}
            if not found or (not native and not found <= cases):
                raise ValueError(f"feature {match[1]} pattern matches no cases or non-case files: {pattern}")
            mapped.update(found)
    if cases - mapped:
        raise ValueError("unmapped case: " + ", ".join(sorted(cases - mapped)))
    for path in (root / "scripts").glob("*.py"):
        if len(path.read_text().splitlines()) > 600:
            raise ValueError(f"script exceeds file line limit: {path.name}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        check(args.root)
    except (ValueError, OSError) as error:
        print(str(error), file=sys.stderr)
        sys.exit(1)
    print("Parity ledger and outside-in feature mappings agree")

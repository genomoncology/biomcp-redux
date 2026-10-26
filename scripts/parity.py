"""Print the parity ledger from the slices' explicit leaf-command accounting."""

import argparse
import json
import re
from pathlib import Path

# Historical inventory: leaf commands and command-specific option slots only.
# Global flags, positional values, aliases and flags on non-leaf parents are excluded.
INVENTORY = "sdlc/planning/parity-0.9.3.json"
OUTCOMES = (
    "Ported or renamed",
    "Dropped by ruling",
    "Deferred by ruling",
    "Not yet decided",
)
START = "<!-- parity-accounting -->"
END = "<!-- /parity-accounting -->"
LEDGER_START = "<!-- parity-ledger -->"
LEDGER_END = "<!-- /parity-ledger -->"
UPGRADE_START = "<!-- upgrade-commands -->"
UPGRADE_END = "<!-- /upgrade-commands -->"
HEADER = "| 0.9 leaf command | Command outcome | Ported or renamed options | Dropped options | Deferred options | Undecided options | Ruling |"


def inventory(root):
    data = json.loads((root / INVENTORY).read_text())
    if data["release"] != "0.9.3" or not data["leaves"]:
        raise ValueError("expected released 0.9.3 leaf inventory")
    return {command: set(leaf["options"]) for command, leaf in data["leaves"].items()}


def rows(root):
    leaves = inventory(root)
    commands = set()
    for path in sorted((root / "sdlc/planning/slices").glob("*.md")):
        text = path.read_text()
        if START not in text:
            continue  # A slice without a closed accounting table remains undecided.
        if text.count(START) != 1 or text.count(END) != 1:
            raise ValueError(f"{path}: expected one accounting block")
        block = text.split(START, 1)[1].split(END, 1)[0].strip().splitlines()
        if len(block) < 3 or block[0] != HEADER:
            raise ValueError(f"{path}: invalid accounting table header")
        for line in block[2:]:
            cells = [cell.strip() for cell in line.strip("|").split("|")]
            if len(cells) != 7:
                raise ValueError(f"{path}: invalid accounting row: {line}")
            command, outcome, *rest = cells
            if not re.fullmatch(r"`[a-z][a-z -]*`", command) or command in commands:
                raise ValueError(f"{path}: invalid or duplicate leaf command {command}")
            if outcome not in OUTCOMES or not rest[-1]:
                raise ValueError(
                    f"{path}: missing command outcome or ruling for {command}"
                )
            name = command.strip("`")
            if name not in leaves:
                raise ValueError(f"{path}: unknown inventory leaf {command}")
            commands.add(command)
            seen = set()
            for cell in rest[:4]:
                options = re.findall(r"`(--[a-z][a-z-]*)`", cell)
                if cell != "—" and cell != ", ".join(
                    f"`{option}`" for option in options
                ):
                    raise ValueError(
                        f"{path}: invalid option slots for {command}: {cell}"
                    )
                for option in options:
                    if option in seen:
                        raise ValueError(f"{path}: duplicate option {command} {option}")
                    if option not in leaves[name]:
                        raise ValueError(f"{path}: unknown option {command} {option}")
                    seen.add(option)
            yield path, cells


def counts(root):
    totals = {outcome: [0, 0] for outcome in OUTCOMES}
    for _, (_, outcome, *rest) in rows(root):
        totals[outcome][0] += 1
        for category, cell in zip(OUTCOMES, rest[:4]):
            totals[category][1] += len(re.findall(r"`--[a-z][a-z-]*`", cell))
    leaves = inventory(root)
    accounted_commands = set()
    accounted_options = set()
    for _, (command, _, *rest) in rows(root):
        name = command.strip("`")
        accounted_commands.add(name)
        for cell in rest[:4]:
            accounted_options.update(
                (name, option) for option in re.findall(r"`(--[a-z][a-z-]*)`", cell)
            )
    totals["Not yet decided"][0] += len(leaves.keys() - accounted_commands)
    all_options = {
        (command, option) for command, options in leaves.items() for option in options
    }
    totals["Not yet decided"][1] += len(all_options - accounted_options)
    return totals


def table(root):
    rows = [LEDGER_START, "| Measure | Count |", "| --- | --- |"]
    for label, values in [
        (
            "0.9 commands and options",
            (len(inventory(root)), sum(map(len, inventory(root).values()))),
        ),
        *counts(root).items(),
    ]:
        rows.append(f"| {label} | {values[0]} commands, {values[1]} options |")
    return "\n".join([*rows, LEDGER_END])


def upgrade(root):
    lines = [
        UPGRADE_START,
        "| 0.9 command | Outcome | Dropped options | Deferred options | Migration ruling |",
        "| --- | --- | --- | --- | --- |",
    ]
    for path, (command, outcome, _, dropped, deferred, _, ruling) in rows(root):
        if outcome != "Not yet decided":
            source = "slices/" + path.name
            lines.append(
                f"| {command} | {outcome} | {dropped} | {deferred} | [{ruling}]({source}) |"
            )
    return "\n".join([*lines, UPGRADE_END])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root", type=Path, default=Path(__file__).resolve().parents[1]
    )
    parser.add_argument(
        "--upgrade", action="store_true", help="Print the upgrade note command block"
    )
    args = parser.parse_args()
    print(upgrade(args.root) if args.upgrade else table(args.root))

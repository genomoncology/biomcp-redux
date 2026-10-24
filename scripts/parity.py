"""Print the parity ledger from the slices' explicit leaf-command accounting."""
import argparse
import re
from pathlib import Path

# Historical inventory: leaf commands and command-specific option slots only.
# Global flags, positional values, aliases and flags on non-leaf parents are excluded.
BASELINE = (112, 375)
OUTCOMES = ("Ported or renamed", "Dropped by ruling", "Not yet decided")
START = "<!-- parity-accounting -->"
END = "<!-- /parity-accounting -->"
LEDGER_START = "<!-- parity-ledger -->"
LEDGER_END = "<!-- /parity-ledger -->"
HEADER = "| 0.9 leaf command | Command outcome | Ported or renamed options | Dropped options | Undecided options | Ruling |"


def counts(root):
    totals = {outcome: [0, 0] for outcome in OUTCOMES}
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
            if len(cells) != 6:
                raise ValueError(f"{path}: invalid accounting row: {line}")
            command, outcome, *rest = cells
            if not re.fullmatch(r"`[a-z][a-z -]*`", command) or command in commands:
                raise ValueError(f"{path}: invalid or duplicate leaf command {command}")
            if outcome not in totals or not rest[-1]:
                raise ValueError(f"{path}: missing command outcome or ruling for {command}")
            commands.add(command)
            totals[outcome][0] += 1
            seen = set()
            for category, cell in zip(OUTCOMES, rest[:3]):
                options = re.findall(r"`(--[a-z][a-z-]*)`", cell)
                if cell != "—" and cell != ", ".join(f"`{option}`" for option in options):
                    raise ValueError(f"{path}: invalid option slots for {command}: {cell}")
                for option in options:
                    if option in seen:
                        raise ValueError(f"{path}: duplicate option {command} {option}")
                    seen.add(option)
                totals[category][1] += len(options)
    # Unlisted units are undecided, including slices whose accounting has not closed.
    for index, baseline in enumerate(BASELINE):
        if sum(value[index] for value in totals.values()) > baseline:
            raise ValueError("slice accounting exceeds the historical inventory")
        totals[OUTCOMES[2]][index] = baseline - sum(totals[key][index] for key in OUTCOMES[:2])
    return totals


def table(root):
    rows = [LEDGER_START, "| Measure | Count |", "| --- | --- |"]
    for label, values in [("0.9 commands and options", BASELINE), *counts(root).items()]:
        rows.append(f"| {label} | {values[0]} commands, {values[1]} options |")
    return "\n".join([*rows, LEDGER_END])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    print(table(args.root))

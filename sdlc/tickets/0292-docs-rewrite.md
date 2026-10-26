# 0292 — Rewrite the user docs for 1.0

Status: OPEN.

Milestone: 5

Depends on: 0275, 0279

## Outcome

README.md becomes a short user guide: what BioMCP is for, how to install it, how to set keys, how to run it over the CLI and MCP, and a few worked questions. Generated reference docs list every command, option and section from the command grammar, and recipes come from harvested case run steps so every documented command runs against recordings with a zero exit.

- Why: milestone 5 delivers generated reference docs, a short hand-written guide and recipes, and the current README is a log of features added one ticket at a time.
- Starts from: README.md, the generated help, `make docs-check`, upgrade.md, the open-by-default and key decisions, and the case run steps.
- Rules: the README describes only released behavior and names no private project or vendor. Commercial users are told to set `"disable_sources": ["noncommercial", "restricted"]`.
- Proof: docs-check runs every documented command against recordings and fails on a planted command that does not exist or exits nonzero.
- Changes: README.md, a generated reference folder, recipes and docs-check. The landing commit also ticks ROADMAP.md's scheduled live replay box, citing 0068 and 0252, which landed without ticking it.
- Defers: the agent skill guide, which is 0187.

# 0292 — Rewrite the user docs for 1.0

Status: OPEN.

Milestone: 5

Depends on: 0275, 0279

## Outcome

README.md becomes a short user guide: what BioMCP is for, how to install it, how to set keys, how to run it over the CLI and MCP, and a few worked questions. Generated reference docs list every command, option and section from the command grammar, and recipes come from harvested case run steps so every documented command runs against recordings with a zero exit.

- Why: milestone 5 delivers generated reference docs, a short hand-written guide and recipes, and the current README is a log of features added one ticket at a time.
- Starts from: README.md, the generated help, `make docs-check`, upgrade.md, the open-by-default and key decisions, and the case run steps.
- Rules: the README describes only released behavior and may name public products and vendors where useful, and leaves out private projects, customers, internal infrastructure and tokens (Ian ruling 2026-10-13, relayed in m8340 and m8360). Commercial users are told to set `"disable_sources": ["noncommercial", "restricted"]`.
- Proof: docs-check runs every documented command against recordings and fails on a planted command that does not exist or exits nonzero.
- Changes: README.md, a generated reference folder, recipes and docs-check. The landing commit also ticks ROADMAP.md's scheduled live replay box, citing 0068 and 0252, which landed without ticking it. Also the one-sentence correction in `skills/SKILL.md`: reads can auto-sync approved bundles such as HGNC and ClinGen within `auto_sync_max_bytes`, replacing the claim that reads never synchronize, with the embedded guide budget remeasured (PM ruling 2026-10-26, m8126 ask 37).
- Defers: the agent skill guide, which is 0187.

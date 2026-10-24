# 0097 — Document the variant commands and close the variant accounting

Status: OPEN.

Milestone: 2

Depends on: 0087
Depends on: 0088
Depends on: 0089
Depends on: 0090
Depends on: 0091
Depends on: 0092
Depends on: 0093
Depends on: 0094
Depends on: 0095
Depends on: 0096

## Outcome

Help for every variant command is generated from the schemas with examples from the cases, the MCP description names them within its ceiling, variant.md's accounting gives every 0.9 variant command, option and section its 1.0 answer under the rulings, the parity ledger counts them, and ROADMAP.md ticks each variant box whose check passes.

- Starts from: variant.md section 6; the rulings; 0047's and 0086's pattern; `scripts/parity.py`.
- Risks the proof must cover: help that implies classification; a retired option with no message; a shipped size over budget without a reason.
- Proof: a help case asserting the identifier forms `get variant` takes and the search step it names for shorthand, with every example running on replies; an MCP `tools/list` case under its ceiling; `make docs-check` with the ledger updated; a reviewer checks every accounting row and ceiling. Then make landing-gate runs through pm job after current main.

# 0086 — Document the trial commands and close the trial accounting

Status: COMPLETE.
Landed: 353846c4c6dc7c7b1b11c2cf1135c0f0dc03e8e4

Milestone: 2

Depends on: 0081
Depends on: 0082
Depends on: 0083
Depends on: 0084
Depends on: 0085

## Outcome

Help for `search trial`, `get trial` and `trial document` is generated from the schemas with examples from the cases, the MCP description names the trial commands within its ceiling, trial.md's accounting table gives every 0.9 trial command, option and section its 1.0 answer under the rulings, the parity ledger counts them, and ROADMAP.md ticks each trial box whose check passes.

- Starts from: trial.md's accounting; the rulings; 0047's help and accounting pattern; `scripts/parity.py`.
- Risks the proof must cover: help that promises eligibility judgment; a retired 0.9 option with no message; a shipped size over its budget without a stated reason.
- Proof: a help case asserts each option, the discovery-only wording for `--gene`, `--variant` and `--criteria`, and that each example parses and runs on replies; an MCP `tools/list` case under its ceiling; `make docs-check` passes with the ledger updated; a reviewer checks every accounting row and every case's ceiling. Then make landing-gate runs through pm job after current main.

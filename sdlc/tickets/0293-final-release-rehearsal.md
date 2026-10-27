# 0293 — Rehearse the final release candidate on every platform

Status: OPEN.

Milestone: 5

Depends on: 0289, 0290, 0292, 0297

## Outcome

The final release candidate's artifacts install and run on every promised platform, repeating the 0066 installed rehearsal: wheel, installer archives, Homebrew formula, container, desktop bundle and registry entry, each running `version`, `health`, one replayed search and get, and stdio and HTTP MCP. The report names any platform that could not be exercised.

- Why: the release plan requires final-candidate installed proof before Ian approves publication.
- Starts from: 0066's rehearsal recipe, `release/gate.py` and `release/matrix.sh`, release.md and the machine registry.
- Rules: the rehearsal includes one 0289 workflow dry run, on a fork tag or workflow_dispatch with publication disabled, that builds every artifact on the five platforms and two containers; Ian approves that external run before it starts. the rehearsal publishes nothing. Machines are booked through pm job. The rehearsal verifies or refreshes the 0290 schema baseline against the selected final candidate, so the published 1.0 schema set becomes the baseline 1.0.1 is compared with; 0290's snapshot is provisional input only (PM ruling 2026-10-26, m8126 ask 31).
- Proof: one report per platform with exact commands, versions and exits, reviewed by a fresh reviewer.
- Changes: none expected beyond fixes the rehearsal finds, each its own ticket.
- Defers: publication, which needs Ian.

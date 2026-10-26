# 0293 — Rehearse the final release candidate on every platform

Status: OPEN.

Milestone: 5

Depends on: 0289, 0290, 0291, 0292

## Outcome

The final release candidate's artifacts install and run on every promised platform, repeating the 0066 installed rehearsal: wheel, installer archives, Homebrew formula, container, desktop bundle and registry entry, each running `version`, `health`, one replayed search and get, and stdio and HTTP MCP. The report names any platform that could not be exercised.

- Why: the release plan requires final-candidate installed proof before Ian approves publication.
- Starts from: 0066's rehearsal recipe, `release/gate.py` and `release/matrix.sh`, release.md and the machine registry.
- Rules: the rehearsal publishes nothing. Machines are booked through pm job.
- Proof: one report per platform with exact commands, versions and exits, reviewed by a fresh reviewer.
- Changes: none expected beyond fixes the rehearsal finds, each its own ticket.
- Defers: publication, which needs Ian.

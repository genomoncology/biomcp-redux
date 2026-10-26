# 0279 — Remove private infrastructure and vendor names from public repo text

Status: OPEN.

Milestone: 5

## Outcome

README.md, the runner README, release scripts, tracked scripts and docs outside `sdlc/` name no private project, team role, machine, workspace path or vendor, so the repository history can replace the public BioMCP repository. Capture scripts take their host and evidence path from arguments or environment instead of hard-coding them, or leave the repository when they served one ticket.

- Why: the milestone 4 review found `pm job`, Jarvis, the workspace machine registry, `ssh yellow` and `/home/ian/workspace/data` paths in tracked public files, and vendor product names in README.md and planning docs, while the workspace rules say public repos talk only about themselves and name no vendor.
- Starts from: README.md lines 18, 26 and 50, `crates/biomcp/tests/runner/README.md` lines 73 to 75, `release/matrix.sh`, the 16 tracked scripts that hard-code a host or workspace path, `sdlc/planning/release.md`, architecture.md's MCP compatibility text and `2026-10-19-public-repos-skip-history-scrubs.md`.
- Rules: describe the search and fetch MCP interface by its tool names and behavior without naming a vendor. A distribution channel that must be named by its real name goes to the rules owner through the PM before it stays. `sdlc/` records may keep internal names as the workspace decision allows.
- Proof: a docs check fails when a tracked file outside `sdlc/` contains a listed private name, machine name or workspace path, and passes after the edits. Capture scripts still run with explicit arguments.
- Changes: the named docs and scripts, and the docs check.
- Defers: nothing.

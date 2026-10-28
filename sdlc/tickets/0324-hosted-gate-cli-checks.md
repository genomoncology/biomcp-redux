# 0324 — Run literal CLI version and offline health in the hosted release gate

Status: COMPLETE.
Landed: b16063bf4efa1a9205dd505b7cd19490f26bf2a0

Milestone: 5

## Outcome

The release workflow's hosted install gate runs the installed `biomcp version` and `biomcp health --offline` with every source disabled on each hosted platform, and records the binary hash and candidate commit with each result.

- Why: 0293's review found the hosted gate checks version through MCP serverInfo and health through HTTP `/health`, never the literal CLI commands, and native Intel macOS has no local machine, so only the hosted run can give its literal CLI proof. PM ruling 2026-10-27 (m8126 ask 63).
- Starts from: `release.yml`, `release/gate.py`, the 0293 rules and the b18 local evidence.
- Rules: an isolated config with all sources disabled, no provider calls, and the existing MCP and HTTP checks kept. A workflow-only change; b18 installed-product evidence carries forward under the 0293 candidate rule with the exact path diff and both hashes.
- Proof: an outside-in synthetic run of the gate shows both CLI rows with binary and candidate identity, and a failing CLI exit fails the gate.
- Changes: the workflow, the gate and a fixture.
- Defers: the external dry run, which waits for Ian's approval.

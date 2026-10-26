# 0294 — Replace the public BioMCP repository history with this repository's

Status: OPEN.

Milestone: 5

Depends on: 0293

## Outcome

After Ian approves the 1.0 release, the public BioMCP repository's history is replaced with this repository's history, keeping its stars, issues and release history, as the repository transition decision says.

- Why: milestones.md makes this the last step of 1.0.
- Starts from: `2026-10-23-a-clean-repo-whose-history-replaces-biomcp.md`, `2026-10-19-public-repos-skip-history-scrubs.md`, 0279's public text check, and the public repository's settings.
- Rules: nothing runs before Ian's explicit approval of this step. A written runbook with a rollback goes to Ian with the approval request. The release operator performs the push.
- Proof: after the push, the public repository shows this history, its stars and releases are intact, and the 1.0 tag matches the approved candidate.
- Changes: none in this repository.
- Defers: nothing.

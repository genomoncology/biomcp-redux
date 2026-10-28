# 0294 — Append this repository's history to the public BioMCP repository

Status: OPEN.

Milestone: 5

Depends on: 0293

## Outcome

After Ian approves, the public BioMCP main gains one merge commit whose parents are its previous main and the approved 1.0 candidate and whose files equal that candidate, so the old history, tags, stars, issues and releases stay and this repository's history follows them.

- Why: Ian ruled on 2026-10-27 to append, not replace (`2026-10-27-append-history-to-biomcp.md`), because the public repository has earlier releases.
- Starts from: that decision, the `release/1.0` candidate in 0293, and the public repository's settings.
- Rules: nothing is pushed to the public repository before Ian's explicit approval of this step. A rehearsal on a scratch clone shows the merge, that old tags resolve and that the merge tree equals the candidate before the approval request. The release operator performs the push without force.
- Release order (PM ruling 2026-10-28, pre-release review `sdlc/records/2026-10-28-pre-release-review.md`): one Ian approval covers the append, the tag and publication. Then the append, then the `v1.0.0` tag on the public merge commit, then the release workflow on that tag, then the operators publish from that run. This ticket owns the tag step.
- Proof: after the push, the public repository shows both histories, v0.9.3 and older tags resolve, stars and releases are intact, and the merge tree equals the approved candidate.
- Changes: none in this repository.
- Defers: nothing.

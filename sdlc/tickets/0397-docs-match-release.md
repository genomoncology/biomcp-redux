# 0397 — Make the user docs describe the released product

Status: OPEN.

Milestone: 5

## Outcome

The upgrade guide, roadmap and slice accounting tables say GenCC sync and the gene `gencc` section ship, and the generated parity ledger and upgrade table are regenerated. The upgrade guide, release notes source, README Develop section and cautions state released fact, with no planned, provisional or undecided wording and no team instructions. Cautions says AlphaGenome takes its key from `key set alphagenome` or `ALPHAGENOME_API_KEY`. The roadmap checklist matches the ticket records, says history is appended, and drops the undecided advanced-filter line.

- Why: pre-release review `sdlc/records/2026-10-28-pre-release-review.md`, docs findings 1, 5, 6 and 7. 0383 shipped GenCC, but the hand-kept slice tables still list it as deferred.
- Starts from: `sdlc/planning/slices/cross-cutting.md`, `sdlc/planning/slices/gene.md`, `scripts/parity.py`, `sdlc/planning/upgrade.md`, `sdlc/planning/release.md`, `ROADMAP.md`, `README.md` and `docs/cautions.md`.
- Rules: regenerate generated views; keep the README to released behavior.
- Proof: `make docs-check` passes; one docs check fails when a leaf marked deferred or dropped appears in the generated command reference.
- Changes: the files above and that one check.
- Defers: nothing.

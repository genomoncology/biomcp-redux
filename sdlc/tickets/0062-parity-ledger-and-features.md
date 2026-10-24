# 0062 — Fill the 0.9 parity ledger and the feature list

Status: COMPLETE.
Landed: 09c32721051b340f8e248bee9bc33affcfb2e3af

Milestone: 1

Changes: Makefile, ROADMAP.md, scripts/, sdlc/features.md, sdlc/planning/slices/gene.md

## Outcome

ROADMAP.md's parity table counts every 0.9 command and option as ported or renamed, dropped by ruling, or not yet decided, and the counts add up to 0.9's 112 commands and 375 options. The gene and article rows come from the accounting tables in the gene and article slice designs. Everything else counts as not yet decided. `sdlc/features.md` lists every feature that has landed, one line each: a stable name and one sentence of behavior in the caller's words. Today the ledger shows 0 decided while the gene accounting box is ticked, and the feature list has one line.

- Starts from: ROADMAP.md "Parity with 0.9"; the 0.9 accounting sections of the gene and article slice designs; ticket 0034; the landed cases under `tests/cases/`; `sdlc/features.md`.
- PM decision: 2026-10-24. The ledger counts live in one generated table. A small script in `scripts/` reads the slice accounting tables and prints the counts, and the docs check fails when ROADMAP.md disagrees with it, so the ledger cannot drift again. Each later slice's accounting ticket updates its table, and the counts follow. Each feature line names the case directory that proves it.
- Proof: the docs check fails on a planted wrong count and passes on the real one. Every `tests/cases/` command directory maps to at least one feature line.
- Defers: the article rows not yet decided, which 0047 settles.

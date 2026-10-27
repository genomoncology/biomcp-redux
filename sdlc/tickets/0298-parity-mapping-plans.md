# 0298 — Declare parity mapping plans for every harvested question

Status: OPEN.

Milestone: 5

Depends on: 0291

## Outcome

Every harvested question has a reviewed parity plan before the whole comparison runs: endpoint declarations for each source that 0.9.3 calls, grounded in the 0.9.3 release, and old-command and per-fact projections, including followed actions, walks and MCP cases. The 0291 runner reports no gaps and no unsafe plans over the whole case set.

- Why: 0291 isolates only the fixed MyGene sample, and the whole run needs declared mappings for the other sources and questions. A build delegate running 0297 must not change code during the run. PM ruling 2026-10-26 (m8126 ask 32).
- Starts from: the landed 0291 runner and its gap and unsafe-plan reports, the parity accounting tables, the 0.9.3 release source and binary, and the harvested cases under `tests/cases/`.
- Rules: released 0.9.3 is the baseline. A plan declares only endpoints the 0.9.3 release uses. A question that cannot be projected to 0.9.3 is reported with its reason and not counted as a 1.0 regression. No live calls are needed to land this.
- Proof: the runner's dry plan over every harvested case reports zero gaps and zero unsafe plans, or names each unprojectable question with its reason, and a reviewer checks a sample of projections against 0.9.3 source.
- Changes: parity plan declarations and the runner fixes they need.
- Defers: running the comparison, which is 0297.

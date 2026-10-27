# 0298 — Declare 0.9.3 endpoints and inventory every harvested case for parity

Status: OPEN.

Milestone: 5

Depends on: 0291

## Outcome

The parity runner declares the endpoints each source uses in the 0.9.3 release and runs a fail-closed dry inventory over every harvested case. The inventory puts each case in one class: a direct CLI step with a reviewed 0.9.3 projection, a multi-step or stateful case for 0299, a case covered by another case's facts, or a question 0.9.3 cannot answer, with its reason. Direct steps get their reviewed per-fact projections here, in entity batches.

- Why: 0291 runs only direct CLI steps with one JSON exchange per capture, and the harvested set also holds followed actions, walks, MCP surfaces, YAML-form files and bundle or stateful cases. PM ruling 2026-10-26 (m8126 ask 38) splits the work so each capability lands with its own end-to-end proof.
- Starts from: the landed 0291 runner and its gap reports, the parity accounting tables, the 0.9.3 release source and binary, and the harvested cases under `tests/cases/`.
- Rules: released 0.9.3 is the baseline. A plan declares only endpoints the 0.9.3 release uses. Parity compares asserted facts, not transports: an MCP case whose facts a CLI case already asserts is counted once through the CLI case, and the 1.0 MCP surface stays proved by its own tests. A question 0.9.3 cannot answer is reported with its reason and not counted as a 1.0 regression. No live calls and no generic framework.
- Proof: the dry inventory over every harvested case assigns every case exactly one class with no unclassified case and no unsafe plan, and a recorded run over one direct-step batch per entity that has positive direct cases produces both verdicts; drug and study positives, which begin with a bundle or cBioPortal sync, are proved in 0299 (PM ruling 2026-10-26, m8126 ask 39). A reviewer checks a sample of projections against 0.9.3 source.
- Changes: endpoint declarations, the inventory, direct-step plans and the runner fixes they need.
- Defers: multi-step and stateful execution, which is 0299, and the whole run, which is 0297.

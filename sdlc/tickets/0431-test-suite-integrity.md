# 0431 — Close the test suite's integrity gaps

Status: OPEN.

Milestone: 5

## Outcome

Every native trial and YAML case maps to a listed feature by its first name segment, and the YAML `feature` field is removed or checked. `make docs-generate` writes docs only and never rewrites case expectations. One trial proves the default config and cache paths with XDG unset. The two never-running tests are deleted or wired into the rehearsal. Expected values come from literals, not product tables. The `covers` promise in architecture 10.2 is built or removed, as are the ignored `harvest` and `verified` fields. Parity, live, capture and measurement tooling moves out of the routine target and coverage. Size budgets live in YAML only. Transport failures are proved once each, with one provider error-shape case per adapter.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`.
- Starts from: `scripts/check_docs.py`, `crates/biomcp/tests/runner`, `crates/biomcp/src/grammar/reference.rs`, `tests/cases`, the budget files and architecture 10.
- Rules: no lost behavior coverage; each removed case names the case that still proves its behavior.
- Proof: planted unmapped trials, a drifted help case and a missing default path each fail.
- Changes: the files above.
- Defers: nothing.

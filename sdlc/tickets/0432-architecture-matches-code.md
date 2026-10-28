# 0432 — Make the architecture document describe the code

Status: OPEN.

Milestone: 5

Depends on: 0428

## Outcome

Architecture sections 2.1 to 2.4, 4.1, 4.2, 4.13, 5.1, 5.7, 6.1, 7.5, 8.7 and 10 describe the built system and link to the authoritative lists instead of restating them. The dependency ratchet table becomes the authority by amending `2026-10-23-crates-enforce-dependency-direction.md`. Per-ticket tables move to module docs. biomcp-query's public surface narrows to its entry points.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. The document still says no crate uses an async runtime, lists files that do not exist and shows a 7-method `Leg`.
- Starts from: `sdlc/planning/architecture.md`, the decision, `crates/biomcp-query/src/lib.rs`.
- Rules: written after 0419 to 0428 so it describes the final shape; no new rules.
- Proof: a reviewer checks each section against the code; `pub` items in biomcp-query outside the entry points are gone.
- Changes: the document, the decision and `crates/biomcp-query/src/lib.rs`.
- Defers: nothing.

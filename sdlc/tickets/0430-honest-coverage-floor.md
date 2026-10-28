# 0430 — Make the coverage floor measure outside-in coverage

Status: OPEN.

Milestone: 5

## Outcome

Counted coverage comes only from the product binary running cases and from true type contracts. In-process runner trials move to the excluded library collection or become CLI cases, `#[path]` product includes leave the test binary, stale and non-type contract targets leave the counted list, and test-only modules leave the denominator. The PM sets the new floor from the measured figure.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. The floor has stayed at 90.41 since 2026-10-24 while it counts library calls made inside the test binary.
- Starts from: `scripts/coverage.sh`, `coverage.floor`, `sdlc/features.md`, `tests/ratchets/placement.rs` and the in-process runner files named in the record.
- Rules: architecture 10.3 is the rule; only the PM sets the floor.
- Proof: the measured binary-only figure is recorded; a planted in-process trial no longer raises the counted figure.
- Changes: the files above.
- Defers: nothing.

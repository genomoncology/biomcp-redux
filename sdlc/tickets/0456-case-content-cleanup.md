# 0456 — Clean case content

Status: OPEN.

Milestone: 5

Depends on: 0431

## Outcome

Each case asserts its feature's promise once. Repeated transport-failure cases collapse to one table per failure kind. Cases that copy their expectations from product tables compare against literals. Tests that never run are run or deleted. The review record names no case, so the lane builds a bounded inventory first and puts it in the commit message. The inventory covers cases whose expected values come from product code, failure cases repeated across sources with the same assertion, and tests that no runner registers.

- Why: the code quality review found tests that restate code, repeat one failure for many sources, or never run.
- Starts from: `tests/cases/` and `crates/biomcp/tests/runner/`.
- Rules: delete a case only when another case fails on the same break, and name that case in the commit. Every kept case keeps its feature. No new assertion style.
- Proof: for each deleted case, a planted break of its behavior fails the named covering case; each test the inventory finds unregistered fails discovery when planted back unregistered; the outside-in coverage figure from 0430 does not drop.
- Changes: `tests/cases/` and runner trial files from the inventory.
- Defers: nothing.

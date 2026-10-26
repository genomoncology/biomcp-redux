# 0290 — Compare generated schemas with the last released copy before release

Status: OPEN.

Milestone: 5

## Outcome

A release gate compares every generated schema with the copy in the last released version and fails when a schema changes incompatibly without a new major version, as milestones.md promises for milestone 5. For 1.0, the baseline is the schema set at the release candidate, recorded so 1.0.1 compares against it.

- Why: milestones.md names this gate, and its earlier owner, 0006, was withdrawn without moving the requirement.
- Starts from: the schema generator and its existing same-major compatibility check, `make docs-check`, the release workflow in 0289 and milestones.md.
- Keeps: the generator's current in-tree compatibility rules.
- Changes: a gate script, its make target and the release workflow step.
- Proof: the gate fails on a planted incompatible same-major change against a recorded baseline and passes on the real tree.
- Defers: nothing.

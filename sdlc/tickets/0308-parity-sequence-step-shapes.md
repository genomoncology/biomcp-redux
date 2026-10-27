# 0308 — Declare the common parity sequence step shapes

Status: OPEN.

Milestone: 5

## Outcome

The 1,392 unmatched sequence steps are grouped by command shape, and the PM sees that table before any declaration work. Declarations are then added for the shapes the PM selects, in order of steps covered, and every remaining step stays a counted, named gap.

- Why: j16434 found 395 sequence cases with 1,392 steps whose command shape has no direct declaration. Under the ask 43 ruling, steps reuse direct declarations by shape, so declaring a shape once covers every step with it. PM ruling 2026-10-27 (m8126 ask 46).
- Starts from: the j16434 inventory, the landed 0299 engine and the direct declarations of 0298 and 0300 to 0306.
- Rules: the shape table is reported to the PM before any declaration is written. Declarations are per shape, never per case. A shape 0.9.3 cannot answer is listed unsupported with its reason. No live calls.
- Proof: the shape table with step counts per shape and family; after declarations, the dry inventory's unmatched step count drops by the steps the selected shapes cover, and a reviewer checks a sample of declarations against 0.9.3 source.
- Changes: shape declarations only.
- Defers: shapes the PM does not select, which stay reported gaps.

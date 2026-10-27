# 0308 — Declare the common parity sequence step shapes

Status: OPEN.

Milestone: 5

## Outcome

The 1,392 unmatched sequence steps are grouped by command shape, and the PM sees that table before any declaration work. Declarations are then added for the shapes the PM selects, in order of steps covered, and every remaining step stays a counted, named gap.

- Why: j16434 found 395 sequence cases with 1,392 steps whose command shape has no direct declaration. Under the ask 43 ruling, steps reuse direct declarations by shape, so declaring a shape once covers every step with it. PM ruling 2026-10-27 (m8126 ask 46).
- Starts from: the j16434 inventory, the landed 0299 engine and the direct declarations of 0298 and 0300 to 0306.
- Rules: the shape table is reported to the PM before any declaration is written. Declarations are per shape, never per case. A shape 0.9.3 cannot answer is listed unsupported with its reason. No live calls.
- Proof: the shape table with step counts per shape and family; after declarations, the dry inventory's unmatched step count drops by the steps the selected shapes cover, and a reviewer checks a sample of declarations against 0.9.3 source.
- Scope ruling 2026-10-27 (m8126 ask 47): the table showed that reuse binds only exact command vectors (995 identities for 1,330 steps, 906 singletons), so per-value records are refused. First, a bounded matcher binds actual positional values to one command shape and to the current case's recordings, keeping strict query identity, current config and captures. Then declarations for source-confined JSON and API shapes in table order, starting with S0003 to S0021 minus unsupported and asset groups. Fulltext and supplement asset operations, follow and walk steps, and native recording gaps stay held. OncoKB gene and schema retrieval steps are unsupported in 0.9.3. The Semantic Scholar open-access filter is unsupported because released 0.9.3 rejects it before HTTP. The trial biomarker and eligibility query mappings and article field phrase order stay held as native gaps, with no equivalence ruling.
- Stop rule: after this ticket, the whole 0297 pass runs and the parity claim is judged over the comparable facts with the full gap table. No further gap-filling ticket is filed unless that table shows a family with no comparable coverage.
- Changes: the value-binding matcher and shape declarations.
- Defers: shapes the PM does not select, which stay reported gaps.

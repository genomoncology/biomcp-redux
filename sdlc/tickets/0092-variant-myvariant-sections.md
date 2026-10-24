# 0092 — Show MyVariant's predictor, conservation and historical COSMIC fields

Status: OPEN.

Milestone: 2

Depends on: 0087

## Outcome

The `predict`, `conservation` and `cosmic` sections return the fields MyVariant carries, each with its component source, version and license, only after a public-tier capture proves the fields.

- Starts from: variant.md (accounting rows for predict, conservation and cosmic; budgets); the rulings; `decisions/2026-10-24-article-keeps-every-source-one-option-away.md` (COSMIC only as MyVariant carries it).
- Risks the proof must cover: no blended score; a record lacking a field is optional, while a field the tier lacks is never offered; COSMIC carries its snapshot version and age; `predictions` returns a retirement message naming `predict`.
- PM ruling 2026-10-24 (m4589): MyVariant's dbNSFP fields come from dbNSFP's academic branch, whose publisher grants no non-academic rights to components such as PolyPhen-2, REVEL and CADD. The `predict` and `conservation` sections offer only components whose own terms are independently verified to permit any use, each with its terms and version, and report every other component as unavailable with reason `restricted_terms`. Offering restricted components labeled noncommercial through 0057's terms and disable groups waits for Ian's ruling.
- Proof: a capture lists the fields per section; rs113488022 cases assert each section's fields and licenses; a record missing a field; MCP parity; ceilings from the design table plus envelope. Then make landing-gate runs through pm job after current main.

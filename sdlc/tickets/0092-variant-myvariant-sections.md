# 0092 — Show MyVariant's predictor, conservation and historical COSMIC fields

Status: COMPLETE.
Landed: aec1212878acbda96656f90d497269bef1cb40aa

Milestone: 2

Depends on: 0087

## Outcome

The `predict`, `conservation` and `cosmic` sections return the fields MyVariant carries, each with its component source, version and license, only after a public-tier capture proves the fields.

- Starts from: variant.md (accounting rows for predict, conservation and cosmic; budgets); the rulings; `decisions/2026-10-24-article-keeps-every-source-one-option-away.md` (COSMIC only as MyVariant carries it).
- Risks the proof must cover: no blended score; a record lacking a field is optional, while a field the tier lacks is never offered; COSMIC carries its snapshot version and age; `predictions` returns a retirement message naming `predict`.
- Ian's ruling 2026-10-24 (`decisions/2026-10-24-sources-are-open-by-default.md`): every dbNSFP component MyVariant carries is offered by default, each labeled with its own `Use`, terms name, version and publisher's page; components whose terms allow only academic use, such as PolyPhen-2, REVEL and CADD, are `noncommercial`. `disable_sources` with `noncommercial` or `restricted` switches those components off inside `predict` and `conservation`, each reported `skipped` with reason `disabled`, while MyVariant's other fields stay on. A component with unverified terms is labeled `restricted` until its terms are recorded.
- Proof: a case with `--disable-sources noncommercial` shows the academic-only components skipped with reason `disabled` and the open ones present; a capture lists the fields per section; rs113488022 cases assert each section's fields and licenses; a record missing a field; MCP parity; ceilings from the design table plus envelope. Then make landing-gate runs through pm job after current main.

# 0359 — Add schema formats and complete schema descriptions

Status: COMPLETE.
Landed: 7f3cf0aa33928fa146672991a7e9b5a3ad2d4453

Milestone: 5

Depends on: 0341

## Outcome

Schema properties holding dates carry `format: date` or `date-time` and links carry `format: uri`. Every schema root has a description. The `ErrorCode` description describes error codes, not "an invalid schema request".

- Why: the docs lead's product findings report (2026-10-27), item C4. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the schema derive attributes on date, partial date and link types, and the schema baseline.
- Rules: `PartialDate` keeps its own pattern, because a partial date is not a full `date`.
- Proof: a check over generated schemas fails on a root without a description or a date or link type without its format; the baseline is regenerated.
- Changes: schema attributes, descriptions and the schema baseline.
- Defers: nothing.

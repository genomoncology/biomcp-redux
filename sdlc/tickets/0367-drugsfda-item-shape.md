# 0367 — Give drugsfda search items the same top-level ID and name as other drug sources

Status: OPEN.

Milestone: 5

## Outcome

`search drug --source drugsfda` items carry top-level `id` and `name` like the other drug sources' items, with the application identity kept.

- Why: the docs lead's second-pass product findings (2026-10-27), item G9. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the drugsfda search item type and the other drug sources' item shapes.
- Rules: PM ruling 2026-10-27 (m10746): `id` is the application number; `name` appears only when every product has the same exact `brand_name` (retained Text values after the existing trim, case-sensitive, m10791), and is left out for zero or differing brands; every product and identity stays. No concatenated or guessed names.
- Proof: a replay case shows top-level `id` and `name`; the baseline is regenerated.
- Changes: the drugsfda item type, adapter, cases and the baseline.
- Defers: nothing.

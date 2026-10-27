# 0371 — Move the remaining adapters to the shared bad-value rule

Status: OPEN.

Milestone: 5

Depends on: 0360

## Outcome

Every adapter site that drops an invalid non-identity source value with `.ok()`, `transpose` or a raw text parse that fails before mapping instead records a problem through 0360's shared helper, and the reply still returns. A file and field inventory classifies each audited site as an invalid-value drop (migrated), a required identity refusal (kept) or legitimate absence (kept), and the ticket records it. Adapters that return plain values without a problem channel gain one.

- Why: the 0360 developer's audit found about 170 `.ok()` and `transpose` sites plus raw text deserialization; 0360 keeps the shared helper and the named fixes, and this ticket finishes the rule (coordinator m10687, PM ruling 2026-10-27). Ian's ruling that 1.0 ships every known fix.
- Starts from: 0360's shared helper and rule, and the audit search list.
- Rules: no text heuristics; required identity fields still fail the item. The coordinator may split this ticket by adapter family into further dependent tickets if the inventory is large; each split remains a 1.0 requirement.
- Proof: the inventory lists every audited site with its class; each migrated family has a case with a bad value that returns the reply with a problem; a ratchet fails on a new unclassified `.ok()` in adapter map code.
- Changes: adapter map and raw modules, their cases and the ratchet.
- Defers: nothing.

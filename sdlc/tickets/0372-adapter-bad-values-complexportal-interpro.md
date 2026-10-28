# 0372 — Apply the bad-value rule to ComplexPortal and InterPro

Status: COMPLETE.
Landed: c6d8cb94fe57578c62cb5a8b2a2b5a3403812ddb

Milestone: 5

Depends on: 0360

## Outcome

ComplexPortal and InterPro optional names and raw fields follow the shared bad-value rule. Each invalid non-identity value is left out with a problem through 0360's shared helper and the reply still returns. Required identity fields still fail the item. The ticket records the family's audited sites, each classed as migrated, identity refusal or legitimate absence.

- Why: Split from 0371 by verified adapter family (coordinator m10687 ask 2, PM ruling 2026-10-27). Ian's ruling that 1.0 ships every known fix.
- Starts from: 0360's shared helper and rule, the 0360 audit list and the family's adapter modules.
- Rules: no text heuristics.
- Proof: each migrated field has a case with a bad value that returns the reply with a problem naming the field; the ratchet from 0360 covers the family.
- Changes: the family's adapter modules and cases.
- Defers: nothing.

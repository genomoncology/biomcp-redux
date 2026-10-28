# 0374 — Apply the bad-value rule to EMA

Status: COMPLETE.
Landed: 47d8f4c87c3cabf3e01e5a71230d644bc4d88284

Milestone: 5

Depends on: 0360

## Outcome

EMA required non-identity fields and item metadata follow the shared bad-value rule. Each invalid non-identity value is left out with a problem through 0360's shared helper and the reply still returns. Required identity fields still fail the item. The ticket records the family's audited sites, each classed as migrated, identity refusal or legitimate absence.

- Why: Split from 0371 by verified adapter family (coordinator m10687 ask 2, PM ruling 2026-10-27). Ian's ruling that 1.0 ships every known fix.
- Starts from: 0360's shared helper and rule, the 0360 audit list and the family's adapter modules.
- Rules: no text heuristics. PM ruling (m10687): invalid EMA bundle metadata stays a source-integrity refusal of the bundle, because it describes the whole bundle; invalid optional item fields are left out with a problem.
- Proof: each migrated field has a case with a bad value that returns the reply with a problem naming the field; the ratchet from 0360 covers the family.
- Changes: the family's adapter modules and cases.
- Defers: nothing.

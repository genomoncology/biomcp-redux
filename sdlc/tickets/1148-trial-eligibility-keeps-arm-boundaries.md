# 1148 — trial-eligibility-keeps-arm-boundaries

Status: COMPLETE.
Landed: 906bf4b6b43fd6aa98d219ec45d324c308545fed

Milestone: 7

## Outcome

A trial's eligibility section keeps each arm's criteria under its own heading, so no arm's heading or limits appear inside another arm's text.

## Why

Found 2026-11-22 by scouts exploring BioMCP for multi-entity questions (about 760 CLI calls on build b7def4651). `biomcp get trial NCT04937062 eligibility` placed one arm's heading inside the previous arm's last exclusion item, and showed the study-wide minimum age (0 months) above an arm whose own criteria say 2 months to 17 years.

## Changes

- The eligibility parser starts a new block at each arm heading in the registry text.
- Study-wide age limits are labelled as study-wide.

## Proof

- Replay of that registry record: each arm's heading starts its own block, and no criterion line crosses arms.

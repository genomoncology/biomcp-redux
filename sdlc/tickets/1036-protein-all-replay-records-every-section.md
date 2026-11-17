# 1036 — protein-all-replay-records-every-section

Status: OPEN.

Milestone: 7

## Outcome

The replayed `get protein P00533 all` step in `tests/cases/protein/native-sections.yaml` has a recorded reply for every source the card reads, and it asserts each section's state, so a section silently coming back "unavailable" fails the case. Any other replayed `all` case with the same gap is found and fixed.

## Why

1029 (2026-11-17) found that step's STRING section was unrecorded, and fixed it; its developer then noticed the domains section is still "unavailable" because no InterPro reply was ever recorded. The step never checked overall status, so the gap passed unnoticed.

## Proof

- Proof: the step asserts every section's state is data (or a stated, source-given empty), and removing any one recording makes it fail.

## Starts from

- Starts from: `tests/cases/protein/native-sections.yaml:65-80` and the replay recordings.

## Keeps

- Keeps: the card's sections and reply shape.

## Defers

- Defers: nothing.

## Changes

- Changes: recordings, the case, and any similar `all` cases found.

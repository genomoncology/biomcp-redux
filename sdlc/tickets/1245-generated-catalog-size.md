# 1245 — generated-catalog-size

Status: COMPLETE.
Landed: 43fa4de4576c469937e32afa7ec3a59a1e6b4d02

Milestone: 10

## Outcome

No file in the repository records the measured size of the default typed `tools/list` catalog, so a change in catalog size never causes a merge conflict. `docs/configuration.md` says the catalog stays under its 196,608-byte ceiling and that `make catalog-size` prints the current figure. The docs check measures the served catalog and fails when it exceeds the ceiling, and still checks every other exact property the guide states.

## Why

PM ruling 2026-11-28: `docs/configuration.md` line 82 records the exact catalog size by hand, and `scripts/check_front_map.py` (lines 110–113) requires it to match. Every ticket that changes the catalog edits that one line, so each later landing conflicts there and needs a rebase and a new gate run. This happened on 1216 and 1224 in one hour and will repeat across the rest of the source reach queue. Fix the cause once, ahead of the remaining landings. PM ruling 2026-11-28 on the route: commit no size at all rather than a generated file, because a number no one commits can never conflict and no reader needs the exact figure in the guide; this needs no change to the lander.

## Evidence

- `pm ticket land` refused 1216 (01:05) and 1224 (01:37) with conflicts in `docs/configuration.md` only, on the catalog size line.

## Proof

- Proof: a script test showing the docs check fails when the served catalog exceeds the ceiling and passes under it; `make catalog-size` prints the figure the typed catalog test reports.

## Starts from

- Starts from: `scripts/check_front_map.py` (the `measured` check), `docs/configuration.md` line 82, the Makefile docs targets.

## Keeps

- Keeps: the ceilings unchanged; every other exact property the guide states stays checked.

## Changes

- Changes: a `make catalog-size` target; the docs check tests the served size against the ceiling instead of a recorded number; the configuration guide states the ceiling and points to the target instead of a hand-written number. No pm change.

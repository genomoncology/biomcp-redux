# 1245 — generated-catalog-size

Status: OPEN.

Milestone: 10

## Outcome

The measured size of the default typed `tools/list` catalog lives in one generated file, not in the prose of `docs/configuration.md`. A make target regenerates it, the docs check fails when the committed file differs from what the target generates, and the landing step regenerates it after rebasing, so a change in catalog size never causes a merge conflict.

## Why

PM ruling 2026-11-28: `docs/configuration.md` line 82 records the exact catalog size by hand, and `scripts/check_front_map.py` (lines 110–113) requires it to match. Every ticket that changes the catalog edits that one line, so each later landing conflicts there and needs a rebase and a new gate run. This happened on 1216 and 1224 in one hour and will repeat across the rest of the source reach queue. Fix the cause once, ahead of the remaining landings.

## Evidence

- `pm ticket land` refused 1216 (01:05) and 1224 (01:37) with conflicts in `docs/configuration.md` only, on the catalog size line.

## Proof

- Proof: a case or script test showing the docs check fails on a stale generated size and passes after the make target regenerates it; two branches that each change the catalog size merge without a conflict once the landing step regenerates the file.

## Starts from

- Starts from: `scripts/check_front_map.py` (the `measured` check), `docs/configuration.md` line 82, the Makefile docs targets.

## Keeps

- Keeps: the check exactly as strict: the recorded size must equal the served size, byte for byte, or the docs check fails. The ceilings are unchanged.

## Changes

- Changes: a generated size file and its make target; the docs check compares against it; the configuration guide points to the file or includes it instead of stating the number by hand; the landing step regenerates the file after its rebase. The lander is pm's code: if regenerating there needs a pm change, file it with the pm team and keep the core side working on its own.

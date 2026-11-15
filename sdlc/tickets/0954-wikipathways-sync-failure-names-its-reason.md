# 0954 — wikipathways-sync-failure-names-its-reason

Status: OPEN.

Milestone: 7

## Outcome

`biomcp sync wikipathways` either succeeds on the M5 or fails with a message that names the cause (which file, which step, what the source returned), and the cause found is fixed or recorded.

## Why

0940's developer (2026-11-14): the sync worked on the M5 in job j40885, then failed four times in a row with no reason given, even with a fresh cache folder, while the same catalog files downloaded fine with curl. A failure without a reason can't be acted on, and `tests/cases/pathway/empty-relations.yaml` starts with that sync and runs in public live checks.

## Proof

- Proof: a live job reproduces or rules out the failure; a case or test shows a failed WikiPathways sync names its cause.

## Starts from

- Starts from: pm jobs j40885 and the later failed 0940 jobs (`pm job list 0940`); the WikiPathways sync adapter in `crates/biomcp-adapters/src/wikipathways/`; the shared bulk sync path in `crates/biomcp-fetch` (0951 is changing its time limit; coordinate if both are open).

## Keeps

- Keeps: the sync's files and cache layout.

## Defers

- Defers: nothing.

## Changes

- Changes: the WikiPathways sync and the shared sync error message, as the cause requires.

## Progress

Build, 2026-11-14.

- Cause found: not the 15-second limit 0951 fixed. On current main (job j41049), `sync wikipathways` succeeded twice from WikiPathways (3,570,917 bytes, 1,131 human rows in each catalog) and failed only with `--record` into a `mktemp -d` folder: exit 5, no reason, nothing written. Every failed 0940 run recorded into such a folder. The catalogs are recorded as binary reply files beside the recording, and that writer refuses any symlinked folder anywhere on the path; on macOS the temp folder `/var/folders/...` is reached through the `/var` symlink, so every binary capture there failed. j40885 worked because it recorded into a folder inside the job checkout; its later reads failed because they recorded into the same, now non-empty, folder (a recording conflict).
- Why no reason: a sync reply named only cache failures in `local_error`; recording failures left it empty, and a conflict at startup gave only the bare "local configuration, cache or recording failed".
- Fix: a `--record` folder is held in resolved form (symlinks in the path the caller named are followed once), so binary captures under a symlinked parent work; a symlink inside the recording folder stays refused. Replay resolves its folder the same way.
- Named cause: `local_error` gains recording values (`recording-conflict`, `recording-io`, `recording-missing`, `recording-exhausted`, `recording-malformed`, `recording-limit`, `recording-non-utf8`), filled for any command whose request fails on its recording, and at startup when the `--record` folder is not empty. Enum widened only; schemas regenerated.
- Measured live on the M5 with the fix (j41058, j41073): `sync wikipathways --record` into a `/var/folders` temp folder completes and writes both catalogs; replaying that folder completes; recording again into the same folder exits 5 with `"local_error":"recording-conflict"`.
- Proof: case `pathway/sync-local-failure` replays the j40885 search-catalog recording metadata with its reply file renamed to `../outside.bin`; the sync exits 5 with `local_error` `recording-malformed`. Test `binary_capture_follows_a_symlinked_parent_folder` in `crates/biomcp-fetch/tests/record.rs` records and replays binary replies through a symlinked parent. New feature line `sync-local-failure-names-cause`.

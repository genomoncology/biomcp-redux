# 1181 — damaged-cache-file-error

Status: COMPLETE.
Landed: 3d36a29edd787f8d93f45df6299917982907b9c3

Milestone: 7

## Outcome

When a cache file is damaged, the error names the file's path and the remedy (for example `biomcp store clean` or deleting that file), and `biomcp store clean` itself still runs.

## Why

The 2026-11-24 adversarial review found that one damaged cache file stops every command, including `store clean`, without saying which file.

## Proof

- Proof: A case with a deliberately damaged cache file shows the path and remedy in the error, and `store clean` succeeding.

## Starts from

- Starts from: the cache open path and the `store clean` command.

## Keeps

- Keeps: An undamaged cache behaves as today.

## Defers

- Defers: Automatic repair.

## Changes

- Changes: cache open error, store clean start-up, cases.

# 0865 — rekey-0856-secret-ignores-after-landing

Status: COMPLETE.
Landed: 47b09b3b9108403d61f30a43b459d3a044006205

Milestone: 7

## Outcome

`make secrets` passes on main after 0856's landing: the 17 `release/secrets.ignore.jsonl` rows that bind ClinicalTrials.gov page tokens in 0856's recordings point at the landed main commit 10f249ed9 instead of the branch commits 504c587ab and d84881acf, with blob, file, line and reason unchanged.

## Why

Landing publishes one commit per ticket, so the branch commits are unreachable from main and the scan stops with history-ignore-unreachable-commit, as after the trial-fields landing that 0842 fixed. 0856's review named this step.

## Proof

- Proof: `make secrets` passes as a pm job on the branch head.

## Starts from

- Starts from: 0842's recipe; the last 17 rows of `release/secrets.ignore.jsonl`.

## Keeps

- Keeps: every other row; each row's blob, file, line and reason.

## Defers

- Defers: making the landing gate run the history scan or rekey rows itself (a pm or gate change, through Jarvis if wanted).

## Changes

- Changes: `release/secrets.ignore.jsonl`

# 0842 — rekey-secret-ignores-after-squash-landing

Status: COMPLETE.
Landed: c8a493e59e9600146f2820c88b92bf9078e6ba16

Milestone: 8

## Outcome

`make secrets` passes on main again.

## Why

The trial-fields landing published one commit per ticket, so the 27 secret-scan ignores it added name branch commits that main does not contain. The scan stops with `history-ignore-unreachable-commit` at row 182. The review had warned of this. The ignored values are public ClinicalTrials.gov pagination cursors in recorded replies.

## Proof

- Proof: `make secrets` fails on the base commit and passes on this one, as a pm job.

## Starts from

- Starts from: `release/secrets.ignore.jsonl` rows 182 to 208.

## Keeps

- Keeps: every row's blob, file, line and reason; only the commit changes, to the landed commit that added the same blob.

## Defers

- Defers: a landing that re-keys or preserves ignores itself is a pm matter, mailed to the pm project.

## Changes

- Changes: `release/secrets.ignore.jsonl`

# 1040 — empty-device-name-search-offers-the-manufacturer

Status: OPEN.

Milestone: 7

## Outcome

A `search diagnostic --source fda-pma` or `--source fda-510k` with `--name` that finds nothing says the name matched the trade or device name only, and offers the same words as `--manufacturer` (the FDA applicant) as a next step.

## Why

The reg-affairs pair run on 2026-11-16 (bench ticket 0061, records `0061-persona-reg-affairs-pair/biomcp-cli/calls.md` turns 26-27) asked for Tempus xT's pathway. `--source fda-pma --name 'Tempus xT'` and `--name Tempus` both returned 0 because openFDA's trade name for P210011 is "xT CDx" (applicant Tempus AI, Inc., approved 2023-04-28). `--manufacturer Tempus` would have found it, but the empty reply offered nothing, so the agent reported Tempus xT as a test with no FDA record. That was the run's one wrong fact (bench `personas/_eval/grading-2026-11-16-p61/notes.md:22`).

## Proof

- Proof: a replayed case runs `search diagnostic --source fda-pma --name Tempus` and expects 0 rows and a next step with `--manufacturer Tempus`. Its second step runs that next step and finds P210011. It fails on the base commit.

## Starts from

- Starts from: the fda-pma and fda-510k search adapters' empty replies, and 1011's empty-section wording.

## Keeps

- Keeps: the name match itself.

## Defers

- Defers: searching the trade name and the applicant in one query.

## Changes

- Changes: the empty reply's next step, a case and its recording, and one release-notes line.

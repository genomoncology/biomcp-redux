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

## Progress

- Measured live 2026-11-17: openFDA PMA `trade_name:"Tempus"` and `trade_name:"Tempus xT"` answer 404 "No matches found!"; `applicant:"Tempus"` counts 6 records, P210011 (xT CDx, Tempus AI, Inc.) and its supplements. 510(k) `applicant:"Tempus"` counts 5 (Tempus ECG-AF, xR IVD and others) while `device_name:"Tempus"` finds 19 unrelated devices.
- Built on the existing `next` steps (0904): an fda-pma or fda-510k search with `--name` and no `--manufacturer` that finds 0 records gets one `configure` step, the same first page with the `--name` words as `--manufacturer` (`DiagnosticSearch::name_as_manufacturer`, checked by the query constructor), and a `request-guidance` notice on the source report: `--name matched the PMA trade name only; the FDA applicant is searched with --manufacturer.` (510(k): "510(k) device name"). The notice is free text in the existing `SourceNotice::RequestGuidance`, so no schema or error-message enum changes and no schema-reviews entry. Code in `crates/biomcp-query/src/diagnostic/device_search.rs`.
- Recorded in pm job j42287 (first try j42286 hit a recording conflict from two commands in one folder): `tests/replies/diagnostic/pma-name-applicant/`. Case `tests/cases/diagnostic/pma-name-applicant.yaml` runs `--name Tempus` (0 rows, the notice and the `--manufacturer Tempus` step) and then that step (6 rows, P210011). One release-notes line under Output migration.
- Cheap job j42289 on 529f57bf6: schemas and command reference regenerate with no change; `diagnostic/` (50) and `mcp-parity/` (66) cases pass; with base a405bc44d's crates the new case fails (`/next` missing). Its `check-cheap` failed on clippy (function length, nested `if`), fixed by moving the step and notice into one helper; gate j42292 on d49ee91d3 failed only that length lint, fixed in 903b947ea.
- Gate j42295 on 903b947ea (branch merged with origin/main): `make --keep-going landing-gate docs-check schema-release-check` passed.

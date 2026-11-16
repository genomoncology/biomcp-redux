# 0995 — trial-results-come-whole-in-one-reply

Status: OPEN.

Milestone: 7

## Outcome

An agent reading a trial's posted results gets every outcome measure in one or a few calls, not one record per page. Pages are sized by what fits the reply, not one record at a time.

## Why

Bench ticket 0052's patient-navigator round (2026-11-16, BioMCP b54b4e8; bench `personas/patient-navigator/runs/2026-11-16/analysis.md:19,31`):

- ELI-002 (NCT04853017)'s posted results came one record a page: 93 records, fetched by the agent's own paging script, so 93 BioMCP calls that the call count doesn't show.
- ClinicalTrials.gov returns all 5 outcome measures in one 65,676-byte reply (`/api/v2/studies/NCT04853017?fields=ResultsSection`).
- Code mode got the same numbers from one `curl`.

This is that round's biggest BioMCP finding, and it got worse since 2026-11-12.

## Proof

- Proof: first find, in Progress, why a results page holds one record. Is it the page size, a per-record byte cap, or the 0823/0824 section shape?
- Proof: a routine case over a recorded NCT04853017 results reply returns all 5 outcome measures in at most 2 calls. It fails on the base commit.
- Proof: replies stay within the existing caps. Any schema change follows the 1.0 rules, and the release notes say what changed.

## Starts from

- Starts from: the trial results section (0823, 0824), its continuation and size limits, the bench analysis above.

## Keeps

- Keeps: 1.0 schema rules; reply caps; whole records (no record cut mid-way).

## Defers

- Defers: distance on site rows, printable export, and telling new trials from edited ones in the watch (the same round's other gaps). Those are separate tickets once the next personas confirm them.

## Changes

- Changes: the trial results section or its paging, one case and its recording, the notes.

# 0864 — device-recall-paging-at-small-offsets

Status: COMPLETE.
Landed: b3a49471c47ca6698b8392ec68dc3663daf31fdb

Milestone: 7

## Outcome

Device-recall search pages at small offsets and page sizes without failing as too large.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: pv-scientist log.md:101: pages failed "too-large" at small offsets.

## Proof

- Proof: a routine case pages device recalls at offsets 0, 10 and 20 with limit 10.

## Starts from

- Starts from: the openFDA device-recall adapter's paging.

## Keeps

- Keeps: totals; continuation rules.

## Defers

- Defers: recall reasons, lots and device links (findings record).

## Changes

- Changes: `crates/biomcp-adapters/src/`, `tests/cases/diagnostic/`

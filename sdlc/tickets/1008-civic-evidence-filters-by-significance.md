# 1008 — civic-evidence-filters-by-significance

Status: OPEN.

Milestone: 7

## Outcome

`get variant … civic` and `get drug … --source civic` take `--evidence-significance` (such as resistance, sensitivityresponse), sent to CIViC like `--evidence-level`, and the profile's count reflects it.

## Why

tumor-board needed T790M's 18 resistance items of 46 and read 15 rows (bench `personas/tumor-board/runs/2026-11-16/analysis.md` 8.3). First confirm CIViC's GraphQL `evidenceItems` takes a significance argument.

## Proof

- Proof: a replayed case for EGFR T790M with `--evidence-significance resistance` asserting the count; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

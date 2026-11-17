# 1023 — ddinter-unlisted-pair-offers-a-label-search

Status: OPEN.

Milestone: 7

## Outcome

When a named drug pair has no DDInter row, the reply's next steps offer `search drug NAME --source openfda-label` for each drug, as every other DDInter interactions reply does after 1013, instead of a direct label read that fails when DDInter's name is not an exact openFDA label name.

## Why

1013's review (2026-11-16) found the direct label read fails for DDInter names such as Paracetamol or Salbutamol (`crates/biomcp-query/src/drug/resolve.rs:346`). 1013 changed the other path and kept this one by its Keeps line.

## Proof

- Proof: the `ddinter-pair` case's unlisted-pair step asserts the search steps; it fails on the base commit.

## Starts from

- Starts from: 1013's next-step code in `crates/biomcp-query/src/drug/ddinter.rs`.

## Keeps

- Keeps: the counts, rows and every other next step.

## Defers

- Defers: nothing.

## Changes

- Changes: that next step, its case, help text if it names the step, one release-notes line.

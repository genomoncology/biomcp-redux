# 0945 — pma-find-every-match-on-long-lines

Status: OPEN.

Milestone: 7

## Outcome

`get diagnostic <PMA> summary --find TEXT` returns every match, including later matches on a long extracted line and wrapped matches after one inside an earlier passage.

## Why

0934's review (2026-11-14): `crates/biomcp-query/src/diagnostic/summary.rs:162` finds only the first match on each extracted line, so a PDF that extracts with few line breaks loses hits; `:165-172` searches the joined line from its start, not from `skip`, so a later wrapped match after a discarded one is missed. P150047's lines are short enough that its case passes.

## Proof

- Proof: a synthetic text unit test (shape, not source data) with two matches on one 6 KB line and a wrapped match after a covered one returns all of them.

## Starts from

- Starts from: `crates/biomcp-query/src/diagnostic/summary.rs:150-180`, `crates/biomcp-query/src/text_find.rs`.

## Keeps

- Keeps: 0934's passage cut (512 back, 3,072 max) and paging.

## Defers

- Defers: nothing.

## Changes

- Changes: the summary find loop and a unit test.

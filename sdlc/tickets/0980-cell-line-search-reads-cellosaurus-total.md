# 0980 — cell-line-search-reads-cellosaurus-total

Status: OPEN.

Milestone: 7

## Outcome

Cell-line search reports Cellosaurus's own match count as its total, instead of a note that the reply gives none.

## Why

0873's live check (2026-11-15, pm job j41438): Cellosaurus's JSON search reply has no count, but its txt format carries `numFound` in the header (2,331 for HeLa). 0873 added a "no total" note; the 0873 Outcome prefers the source's own count.

## Proof

- Proof: first check live whether the JSON reply can carry the count (a parameter or header) without a second request; if not, measure the cost of one txt request with `rows=0`.
- Proof: `tests/cases/cell-line/search-total.yaml` asserts the source's total on a recorded reply; it fails on the base commit.

## Starts from

- Starts from: 0873's `crates/biomcp-query/src/cell_line/mod.rs` note and the Cellosaurus adapter.

## Keeps

- Keeps: one request per search unless the measured cost of a second is small; the reply schema.

## Defers

- Defers: nothing.

## Changes

- Changes: Cellosaurus search adapter, cell-line search total, its case.

## Progress

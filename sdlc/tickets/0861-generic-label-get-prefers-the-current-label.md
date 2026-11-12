# 0861 — generic-label-get-prefers-the-current-label

Status: OPEN.

Milestone: 7

## Outcome

`get drug NAME label` given a generic name reads the current originator label (or the label with the most indications) and says which it chose and why, or warns that several labels exist and lists them.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: pt-committee log.md:36: `get drug rivaroxaban label` silently picked a generic label with 2 of 10 indications.

## Proof

- Proof: a routine case with recorded labels for rivaroxaban shows the XARELTO label chosen and the choice disclosed.

## Starts from

- Starts from: drug label resolution after 0815 and 0851.

## Keeps

- Keeps: label ID reads unchanged; exact-match rules of 0851.

## Defers

- Defers: label version history.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `tests/cases/drug/`

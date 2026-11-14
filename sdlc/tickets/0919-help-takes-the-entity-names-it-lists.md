# 0919 — help-takes-the-entity-names-it-lists

Status: OPEN.

Milestone: 7

## Outcome

Every entity name that `help questions` prints works as a help target, so `help adverse_event` answers as `help adverse-event` does.

## Why

In the pharmacovigilance persona's round 2 (bench `personas/pv-scientist/runs/2026-11-13-r2/`, log.md:7-8), `help questions` listed the group `adverse_event`. `help adverse_event` then failed with `usage.unknown_command`, and the agent needed another read to find `help search adverse-event`. Help reads are BioMCP's commonest wasted calls (paper W3).

## Proof

- Proof: a help case for each multi-word entity in `help questions` (such as `adverse_event`, `cell_line`), asserting the underscore form gives the same help as the hyphen form.

## Starts from

- Starts from: the help router; `help questions`.

## Keeps

- Keeps: today's help text and size caps.

## Defers

- Defers: nothing.

## Changes

- Changes: the help router, `tests/cases/help/`.

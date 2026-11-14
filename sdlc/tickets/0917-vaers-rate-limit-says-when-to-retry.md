# 0917 — vaers-rate-limit-says-when-to-retry

Status: OPEN.

Milestone: 7

## Outcome

When CDC WONDER rate-limits a VAERS table, BioMCP tries once more after a short wait, within the source's time limit. If that also fails, the reply says when to retry.

## Why

In the pharmacovigilance persona's round 2 (bench `personas/pv-scientist/runs/2026-11-13-r2/`, log.md:35), the VAERS age table failed with `reason:"rate-limited"` and the notice "retry after the reported delay". It gave no delay. The agent's own retry about 25 s later worked. The persona asked for "a queued retry or a `--retry-after` hint in `next`" so the tables work "in a live committee meeting".

## Proof

- Proof:
  - a synthetic WONDER rate-limit reply followed by a recorded success, where the call succeeds;
  - two rate-limit replies, where the reply fails and names a retry time.

## Starts from

- Starts from: the VAERS adapter; 0909's command budget for slow sources.

## Keeps

- Keeps: honest failure when the retry fails; 0909's budgets.

## Defers

- Defers: retries for other sources.

## Changes

- Changes: the VAERS adapter, `tests/cases/adverse-event/`.

# 0901 — device-recall-rows-carry-reason

Status: OPEN.

Milestone: 7

## Outcome

A year of a manufacturer's device recalls can be scanned in one call: each device recall search row carries the recall's reason (openFDA's `reason_for_recall`), and the device recall search accepts the same firm filter name as the drug recall search.

## Why

In reg-affairs round 2 (biomed-agent-bench `personas/reg-affairs/runs/2026-11-13-r2/log.md:215-245`, report Q6), device recall rows had only classification, status, event number, product code and initiation date. The agent needed one `get` per recall to learn why it was recalled, though openFDA's `device/enforcement` returns `reason_for_recall` on every row. `--firm` (drug recalls) was refused on device recalls, which take `--manufacturer`.

## Proof

- Proof: a recorded case, a device recall search for a 2025 window (Abbott or Roche), where every row carries `reason` from the recorded openFDA reply. The same search with `--firm` gives the same rows as `--manufacturer`.

## Starts from

- Starts from: the device recall search adapter and its row type; 0877's recall card fields.

## Keeps

- Keeps: `--manufacturer`; the card's fields; row size within the existing measured limits, re-measured if the reason pushes them over.

## Defers

- Defers: a hazard or health-hazard-evaluation field, which openFDA does not publish.

## Changes

- Changes: the device recall adapter and query, `crates/biomcp-types/` (an optional row field and an added filter name), `tests/cases/diagnostic/`, `docs/`.

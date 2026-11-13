# 0901 — device-recall-rows-carry-reason

Status: OPEN.

Milestone: 7

## Outcome

A year of a manufacturer's device recalls can be scanned in one call: each device recall search row carries the recall's reason (openFDA's `reason_for_recall`), and the device recall search accepts the same firm filter name as the drug recall search.

## Why

In reg-affairs round 2 (biomed-agent-bench `personas/reg-affairs/runs/2026-11-13-r2/log.md:215-245`, report Q6), device recall rows had only classification, status, event number, product code and initiation date. The agent needed one `get` per recall to learn why it was recalled, though openFDA's `device/enforcement` returns `reason_for_recall` on every row. `--firm` (drug recalls) was refused on device recalls, which take `--manufacturer`.

Narrowed 2026-11-13 by PM decision. 0897 (device recall search by device name) already added `reason` and `product_description` to every row backed by a recall record (review of 0897, branch 8bab2e649). What remains:
- `--firm` as another name for `--manufacturer` on device recalls;
- the reason on enforcement-only rows under `--classification`. `crates/biomcp-adapters/src/fda_device_recall/map.rs:148-159` (`bare`) gives those rows no reason, though openFDA's enforcement record carries `reason_for_recall` (the `Enforcement` struct in `raw.rs:23` doesn't read it);
- 0897's non-ASCII refusal says "letters outside ASCII" but refuses every non-ASCII character (`crates/biomcp-types/src/response/error.rs:191`, `crates/biomcp-types/src/query/diagnostic/search_validation.rs:29`). Turn non-ASCII symbols (®, ™, µ) into spaces like the other refused punctuation, so a name copied from a label ("Alaris® pump", 49 recalls as "Alaris pump") works, and refuse only non-ASCII letters;
- 0897's name notice (`crates/biomcp-query/src/diagnostic/recall.rs:89`) should say the replaced characters became spaces, so `A|B` searches the phrase "A B".

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

# 0897 — device-recall-search-by-device-name

Status: OPEN.

Milestone: 7

## Outcome

A device recall search accepts a device name or phrase (as openFDA's recall product description and device name allow), or resolves it to product codes, so a quality team asking about "infusion pumps" needs no product code first.

## Why

In the pv-scientist persona run of 2026-11-13 (biomed-agent-bench `personas/pv-scientist/runs/2026-11-13/log.md` and `report.md`, analysis beside them; code-mode run in `runs/2026-11-13-raw/`), `--name "infusion pump"` on fda-device-recall was refused and the agent found product code FRN with an openFDA `count=product_code` curl first (report Q8). The code-mode agent searched the product description and got 87 recalls in one call.

## Proof

- Proof: a recorded case where an infusion-pump recall search by name returns the recorded total with class, firm and reason on each row, and the reply says which field the phrase matched.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_device_recall/`.

## Keeps

- Keeps: product-code search.

## Defers

- Defers: a general device-name to product-code resolver across device sources.

## Changes

- Changes: `crates/biomcp-adapters/src/fda_device_recall/`, `crates/biomcp-types/`, `tests/cases/diagnostic/`, `docs/`

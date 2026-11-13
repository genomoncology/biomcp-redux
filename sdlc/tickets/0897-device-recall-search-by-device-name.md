# 0897 — device-recall-search-by-device-name

Status: COMPLETE.
Landed: 8bab2e649dc0f95dd58885ae770745f554968ea8

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

## Progress

- 2026-11-13: Measured live. On openFDA, `product_description:"infusion pump"` over 2021-11-13 to 2026-11-13 counts 87 in both device recall and device enforcement, the code-mode run's number. Without dates the phrase counts 517 recall records and `product_code:FRN` 721, with 309 in both; `"infusion pumps"` counts 191 (no stemming). Inside a quoted phrase openFDA answers HTTP 400 to `! # $ ; ? ^ | ~`, the backtick and letters outside ASCII; a space in their place finds the same words.
- 2026-11-13: `search diagnostic --source fda-device-recall --name TEXT` now matches the phrase on `product_description` (recall records, or enforcement records under `--classification`). Refused punctuation is spaced; a letter outside ASCII is refused with a usage message. The page carries `source_query` with the exact openFDA expression, and a notice names the field and says a name total differs from a product-code total. Search rows add the recall's product description and reason. Recorded case `tests/cases/diagnostic/recall-name.yaml` (pm job j39528) asserts total 87 with class, firm and reason on each row. `docs/cautions.md` explains the match.

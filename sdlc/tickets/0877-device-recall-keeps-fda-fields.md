# 0877 — device-recall-keeps-fda-fields

Status: OPEN.

Milestone: 7

## Outcome

Device recall records keep the seriousness fields openFDA publishes: voluntary or mandated, how customers were notified, product quantity, and the classification date.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5), log.md:182-190: for Z-1747-2025 openFDA's enforcement record carries `voluntary_mandated`, `initial_firm_notification`, `product_quantity`, `center_classification_date` and `report_date`, which BioMCP's card drops; the persona's question is what seriousness signal survives on the device side.

## Proof

- Proof: a routine case for `get diagnostic Z-1747-2025 --source fda-device-recall` showing those fields from a recorded reply.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_device_recall/` and the recall card type.

## Keeps

- Keeps: 0864's reply limit; the RES fields already joined (root cause, PMA numbers).

## Defers

- Defers: the same fields on search rows unless cheap.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-types/src/`, `tests/cases/diagnostic/`

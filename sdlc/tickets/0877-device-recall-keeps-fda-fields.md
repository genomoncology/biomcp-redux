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

## Progress

- Developer note: 2026-11-13. Live check: openFDA's device enforcement record for Z-1747-2025 carries `voluntary_mandated` "Voluntary: Firm initiated", `initial_firm_notification` "Letter", `product_quantity` "4 units", `center_classification_date` 20250512 and `report_date` 20250521 (dates as `YYYYMMDD`). In 1,000 recent device enforcement records, `voluntary_mandated` and both dates were always filled; `initial_firm_notification` was an empty string in 51 and `product_quantity` in 22.
- Change: `get diagnostic … --source fda-device-recall` already made one exact enforcement request for the classification; that same reply now also fills five optional card fields: `voluntary_mandated`, `initial_firm_notification`, `product_quantity`, `classified` (center_classification_date) and `reported` (report_date). An empty native string gives no field and no problem; a misspelled date gives a field problem. The drug recall adapter's `YYYYMMDD` reader is shared rather than copied. Schema change is optional fields only. Root cause, PMA numbers and the 0864 page limit are unchanged.
- Proof: `tests/cases/diagnostic/recall-fields.yaml` replays the live Z-1747-2025 replies recorded in job j38969 (projected, provenance in `tests/replies/diagnostic/recall-fields/README.md`); the synthetic recall case now checks empty strings drop silently.
- Deferred: search rows still carry only the classification. Adding the fields there would change the search row shape and every recall search case's byte budget, so it stays out of this ticket.

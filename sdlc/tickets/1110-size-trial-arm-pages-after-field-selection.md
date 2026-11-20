# 1110 — size-trial-arm-pages-after-field-selection

Status: OPEN.

Milestone: 7

## Outcome

A projected trial arm page counts the bytes of the selected fields against its reply limit and reports when that limit still forces a shorter page.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get trial NCT06305754 NCT07895992 NCT05347134 NCT06132958 NCT06356311 NCT06670196 NCT06422143 NCT06074588 NCT06448312 NCT06459180 NCT06279364 NCT06312176 NCT07318558 NCT06952504 NCT06824467 NCT07419295 NCT06393374 NCT06711900 NCT07829783 NCT07216703 --section arms --fields=record,data.kind,data.label,data.intervention_names --limit 20` returned `"returned":7,"total":10` for NCT06952504 despite `--limit 20`. The same call returned 10 of 12 for NCT07829783 and 9 of 10 for NCT07216703. `biomcp get trial NCT05870319 NCT06170788 NCT07071337 NCT06312137 NCT06966700 NCT06081959 NCT07628504 NCT06841354 --section arms --fields=record,data.kind,data.label,data.intervention_names --limit 20` returned `"returned":11,"total":12` for NCT06966700. Continuations are present. Full rows are sized before field selection, so discarded descriptions take up page space.

## Proof

- Proof: Replay the reported projection against saved rows where the compact first page fits all ten. Also check a true byte-limited page, its reason, and a continuation without gaps or repeats.

## Starts from

- Starts from: `crates/biomcp-query/src/trial/get/page.rs` and `crates/biomcp/src/fields/narrow.rs`.

## Keeps

- Keeps: The byte limit, full rows when no selection is requested, and exact totals remain in place.

## Defers

- Defers: The compact intervention-focused reply belongs to the separate arm ticket.

## Changes

- Changes: Trial arm paging, field selection and cases.

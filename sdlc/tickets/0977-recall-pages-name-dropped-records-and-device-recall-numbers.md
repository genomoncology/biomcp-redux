# 0977 — recall-pages-name-dropped-records-and-device-recall-numbers

Status: OPEN.

Milestone: 7

## Outcome

A drug recall page that drops records without a usable recall number says how many it dropped and names each by openFDA `event_id`, and a device recall page survives a record with a blank or `N/A` recall or product number the same way.

## Why

0975 review (2026-11-15):

- `crates/biomcp-adapters/src/fda_drug_recall/map.rs:37` reports a dropped record as one `value.unknown` problem with no record, and `mod.rs:133-136` deduplicates it, so two dropped records read as one and an agent cannot reach either recall. 1.0.0 allows an optional page field or a widened problem code.
- `crates/biomcp-adapters/src/fda_device_recall/raw.rs:9,24` require well-formed `product_res_number` and `recall_number`; one bad record would fail a whole device recall page, including its joined enforcement lookup (`mod.rs:173`).

## Proof

- Proof: first check live in a job whether device recall pages carry such records today, and record one if found.
- Proof: a routine case where a drug recall page drops two records shows both event IDs; a device recall case with a bad number returns the other rows as partial. Both fail on the base commit.

## Starts from

- Starts from: 0975's drug recall change; `fda_device_recall/raw.rs`, `map.rs`, `mod.rs`.

## Keeps

- Keeps: 1.0.0 schema rules (optional fields, widened enums only); `get drug D-…` and device recall lookups by number stay strict.

## Defers

- Defers: nothing.

## Changes

- Changes: drug and device recall adapters, their cases.

## Progress

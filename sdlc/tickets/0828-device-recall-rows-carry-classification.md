# 0828 — device-recall-rows-carry-classification

Status: OPEN.

Milestone: 8

## Outcome

Device recall search rows show the recall class, and `--classification` filters them, as drug recall rows do.

## Why

Drug recall rows have the class; device rows set it to none although `get` already joins the enforcement record that holds it. Replaces idea 0818.

## Proof

- Proof: a routine case under `tests/cases/diagnostic/` asserts the class on a recorded device recall page and a filtered page; it fails on the base commit.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_device_recall/map.rs`, which sets the class to none, and the enforcement join in its `mod.rs` that `get` uses; the drug recall adapter as the pattern.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. At most one enforcement request per page, not one per row; a row with no enforcement match shows no class and is counted in a notice.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/fda_device_recall/`, `tests/cases/diagnostic/`

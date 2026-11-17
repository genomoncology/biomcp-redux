# 1024 — assertion-rows-carry-their-record-in-one-place

Status: OPEN.

Milestone: 7

## Outcome

Every variant assertion row, mapped or unmapped, carries its source record id and submitter at one fixed row-level path, so a reader of any row finds them in the same place. The help and release notes say what a ClinVar SCV row is: one row per submitted record per classification area.

## Why

The tumor-board run's jq read `statement.attribution.record` and failed on an unmapped submitter row, whose record sits at `unmapped.attribution.record`, losing a page (bench `personas/tumor-board/runs/2026-11-16/analysis.md` 9.1; calls.md 1694, 1822). 1009 added `--kind` and left this; its review (2026-11-16) noted an unmapped RCV row on the same page and that with `--limit 50` one error now loses 50 rows. The same review found "one row per submitter" in 1009's help is loose (`crates/biomcp-adapters/src/clinvar/map.rs:93-95`).

## Proof

- Proof: the `assertions-clinvar-kind` case asserts the row-level record on the unmapped scv row (row 40 of page one) and on a mapped row; the schema check accepts the added optional fields.

## Starts from

- Starts from: `crates/biomcp-types/src/response/variant_assertions.rs:45`, which already reads either place.

## Keeps

- Keeps: the existing statement and unmapped bodies.

## Defers

- Defers: nothing.

## Changes

- Changes: the assertion row type and schema, the case, help text, one release-notes line.

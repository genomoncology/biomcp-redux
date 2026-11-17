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

## Progress

- Branch `ticket/1024-assertion-record` starts from 1009's branch (`ticket/1009-clinvar-kind`, not yet on main) and carries origin/main through 1014.
- Changed: every `VariantAssertion` row (ClinVar and ERepo, mapped or unmapped) now carries optional row-level `record` and `submitter`, copied from the row's attribution in the two shared row builders (`crates/biomcp-adapters/src/assertion.rs`). `statement.attribution` and `unmapped.attribution` are unchanged. Two optional reply fields; schemas regenerated.
- Wording: the SCV row is "one row per submission per classification area" (germline, somatic impact, oncogenicity), checked against `clinvar/map.rs` `submission()`, which emits one row per classification area of each submission. Used in `--kind` and `--clinvar-kind` help and in the 1009 release-notes line; one new release-notes line names `record` and `submitter`.
- Proof: `tests/cases/variant/assertions-clinvar-kind.yaml` page one (`--kind scv --limit 50`, recorded ClinVar 37636 reply) asserts row 0 (SCV000145310, BIC, mapped) and row 40 (SCV000484957, GeneReviews, "not provided", `statement_unmapped`) carry `record` and `submitter` at the row level, beside `statement.attribution.record` and `unmapped.attribution.record`.
- Sizes: `help variant assertions` measured 2,246 bytes in j42109; cap raised from 2,236 to 2,583 (measured plus 15%, per sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md). `help get variant` 6,813 under 1009's 7,824 cap; typed `get_variant` stays under 2,400 (the longer "submitted record" wording measured 2,401 in j42107, so the shorter "submission" wording is used).
- Checks: j42092, j42101, j42107, j42109 (schema and docs regeneration, check-cheap, help/, variant/ and mcp-parity cases). j42109 passed all on 5f9d6baa1.

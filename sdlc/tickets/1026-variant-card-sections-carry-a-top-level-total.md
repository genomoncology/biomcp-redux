# 1026 — variant-card-sections-carry-a-top-level-total

Status: COMPLETE.
Landed: 500066a4175cb34a8930b4305f9e6f102cafa5bb

Milestone: 7

## Outcome

The variant card sections assertions, CIViC, hotspots, OncoKB and GWAS carry their whole-set count as a top-level `total`, equal to their nested `page.total`, and 1022's schema check treats any section with a counted page as holding rows, so no section that counts a set lacks a top-level `total`.

## Why

1022's review (2026-11-16) found its check keys only on an `items` field (`crates/biomcp-types/tests/schemas/section_totals.rs:61`), so these five sections, whose rows sit under `assertions`, `profiles`, `records` and `associations`, still keep their count only in `page`. An agent reading `.data.total` there repeats the comp-bio mistake (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 10.1).

## Proof

- Proof: the widened check fails on the base commit naming these five types and passes after; the variant cases assert each top-level total equals the nested one; `schema-release-check` accepts the added optional fields.

## Starts from

- Starts from: 1022's `section_totals.rs` and the five variant section types.

## Keeps

- Keeps: the nested `page.total`.

## Defers

- Defers: nothing.

## Changes

- Changes: the five section types and schemas, the check, variant cases, one release-notes line.

## Progress

- Survey: walking every checked-in reply schema for card sections whose data holds `items` or a `page` and no top-level `total` found exactly the ticket's five types, `VariantAssertions`, `VariantCivic`, `VariantHotspots`, `VariantOncokb` and `VariantGwas`, in `variant-card`, `variant-page` and `variant-population-result` (and `variant-assertions-result` as the standalone reply). That is the base-commit failure the widened check reports.
- Change: each type gains an optional `total` (serde default), placed before its rows, set wherever its `page` is built or rebuilt (CIViC and OncoKB adapters, the OncoKB section slice, hotspots `new`, assertions paging, GWAS section). `page.total` stays. `variant assertions` therefore carries `result.total` too.
- Check: `sections_with_rows_and_no_total` in `crates/biomcp-types/tests/schemas/section_totals.rs` now treats a section whose data has `page` as holding rows, whatever the rows are called; it proves itself on two planted schemas (`items`, and `page` beside `records`).
- Proof: cases assert top-level `total` equal to `page.total`: `variant/assertions-clinvar-kind` (58, 58, 13, 72 on the reply; 58 on the card's ClinVar section), `variant/hotspots-caid` (3, and within the first 1200 bytes), `variant/civic-section-limit` (11 both steps) and `variant/native-civic` (40); CIViC values read from the replays in pm job j42175. New case `variant/population/gwas-section-total` reads `get variant rs1801133 gwas --section-limit 2` (total 50, 2 rows) from a GWAS Catalog reply recorded live in pm job j42173 (2026-11-17), noted in `tests/replies/variant-population/README.md`. OncoKB is licensed, so its proof is the existing synthetic TLS runner (`crates/biomcp/tests/runner/oncokb.rs`), whose section walk now checks `total` on every page. The response key-order test lists `total` before rows for hotspots and OncoKB.
- Cheap job j42175 on 5bb0f7b78: schemas regenerated (four variant schemas, committed), docs unchanged, `check-cheap`, biomcp-types schemas/response/rust_api tests, `schema-release-check`, and the `variant` and `mcp` cases passed. Typed tool budgets measure input schemas and descriptions, which this change does not touch.
- Gate j42179 on 2951d687b failed only `card_ids_requested_and_statement_subjects_share_published_text` in `crates/biomcp-types/tests/variant_values.rs`: its synthetic eRepo assertions section round-trips and lacked the new `total`; the fixture now states it (1).
- Gate j42184 on 6127641be (contains origin/main 0e3814c80): `make --keep-going landing-gate docs-check schema-release-check` passed; the schema comparison against the 1.0.0 baseline accepts the added optional fields.

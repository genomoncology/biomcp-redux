# 0942 — inner-page-totals-come-before-rows

Status: OPEN.

Milestone: 7

## Outcome

The reply types that still write their own page or total after their rows do so before them: variant hotspots and OncoKB pages, study pages, GWAS association and study pages, and the pages built by the shared `page!` macro in `response_page.rs` (study, pathway and others). Field order only, as 0937 and 0939 did.

## Why

0939's review (2026-11-14): 0939 moved the envelope's `page` and the drug and adverse-event totals ahead of their rows, but these families' own page blocks still follow their rows, so an agent reading only the start of a reply sees rows without their count. The dx-lab-director persona's first-page misreads (bench `personas/dx-lab-director/runs/2026-11-14/analysis.md`) are the reason for the series.

## Proof

- Proof: 0939's `starts_within` runner check on one recorded case per family, and a contract test per type in `crates/biomcp-types/tests/response.rs`, each failing on main.

## Starts from

- Starts from: 0939's change, its `totals_first` helper and `crates/biomcp/tests/runner/assertions/offsets.rs`; `response_page.rs`; the variant, study and GWAS response types.

## Keeps

- Keeps: no schema file changes; MCP text order (alphabetical).

## Defers

- Defers: nothing.

## Changes

- Changes: field order in those types, tests.

## Progress

- Measured on origin/main c5d1fe8b2 (job j40883) and on the first branch commit (j40884, 2c1475f13), replaying the recorded cases; reply sizes are unchanged. Byte offset of the inner total, before → after:
  - `search gwas --pubmed 17554300 --limit 50` (`GwasPage`, 9,514 bytes): `result.page.total` 9,254 → 306.
  - `pathway events R-HSA-5673001 --limit 50` (`PathwayEvents`, a `page!` macro type, 12,402 bytes): 11,910 → 308.
  - `pathway genes R-HSA-5673001` (`PathwayGenes`): 1,419 → 305. `search pathway Hemostasis` (`PathwayPage`): 775 → 178.
  - `get variant CA123643 hotspots` (`VariantHotspots`): section `page` 1,218 → 729.
  - `get cell-line CVCL_0004 xrefs --section-limit 25` (`Capped` inside `CellLineXrefs`): `items.total` 4,566 → 249.
  - `search study Atlas` (`StudyPage`, synthetic, one row): `result.page.total` 419 → 291; the first row now starts at 403.
  - `protein domains P00533` (`ProteinDomains`, one row): `native_total` 533 → 277.
- Changed field order only (no field added, removed or renamed; no schema file changed). The page or total now comes before the rows in `VariantHotspots`, `VariantOncokb`, `AlphaGenomeScorerPage`, `StudyPage`, `StudyFiles`, `StudySamples`, `GwasPage` (with `qualifying_total`), `PathwayPage`, `PathwayGenes`, and in the `page!` macro: study and pathway pages put `page` in the leading block, diagnostic pages put `count_unit` and `distinct_id_total` there. The outcome covers every reply type that still wrote its total after its rows, so the same scan also moved `Capped` (gene pathways, cell-line sections), `TrialRecords` (`offset`, `total`), `GenePage.regions_total` (before `regions`), `ProteinDomains` and `ProteinComplexes`. Adapter-internal pages that are never written as replies were left alone.
- Not changed, because the schema file would change: a generated schema lists `required` in field declaration order (properties are sorted, `required` is not). Where both the rows and the total are required, moving the total rewrites the `required` array: the gate's first run (j40891) failed `checked_in_schemas_match_types` on `gwas-study-page.json`. These types keep their order: `GwasStudyPage` (`items`, `page`), `StudyMutations` (`items`, `page`), the cell-line section pages' `offset` after `items`, and `ProteinSectionPage` (`items`, `total`). On main `search gwas --pubmed 17554300 --studies --limit 50` writes `result.page.total` at byte 1,934 of 2,429. Moving them needs a schema text change (the same constraints, reordered) or a sorted `required` in schema generation, which rewrites many schema files; that is a decision for the PM.
- Tests: `crates/biomcp-types/tests/response.rs` adds one test per family; its helper `response/inner_totals.rs` (kept apart for the 600-line file limit) reads each struct's declared keys, skipped empty rows included, so no row is invented. The tests, one per family (variant, study, gwas, pathway and diagnostic, other bounded collections) covers every moved type, and each fails on main. Recorded cases assert `starts_within`: `gwas/studies.yaml` (1,200), `pathway-sections/examples.yaml` (events and genes, 1,200), `variant/hotspots-caid.yaml` (1,200), `cell-line/k562-xrefs.yaml` (1,200), and the small synthetic replies `study/examples.yaml` (400) and `protein/examples.yaml` (300), each set below its main offset. OncoKB, diagnostic, trial, gene-region and AlphaGenome replies have no recorded case whose total fell late, so they rest on the contract tests.

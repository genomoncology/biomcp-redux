# 0930 — fda-device-search-by-intended-use-text

Status: COMPLETE.
Landed: d90bcb5d7360656c7cf91b97ba5cd595ebff867d

Milestone: 7

## Outcome

`search diagnostic` over FDA PMA and 510(k) can match the device's intended-use text as FDA publishes it (openFDA's approval-order statement for PMA; the 510(k) equivalent where openFDA has one), so "which cell-free DNA assays went through FDA" is one call. The reply says which field matched. Trade-name search stays as today.

## Why

The dx-lab-director persona tried trade-name searches; 'liquid biopsy' and 'cfDNA' got 0 or 1 hit in PMA and 510(k), and it never tried 'cell-free' (log.md:85-92); code mode searched openFDA's `ao_statement:"cell-free"` and found 12 PMA records, including Shield P230009 and SimpleScreen CRC P250029, which the BioMCP agent never saw. Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`.

## Proof

- Proof: a recorded openFDA PMA reply for an intended-use phrase (e.g. "cell-free") returns Shield P230009 and names the matched field; the same phrase as a trade name returns what it does today.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_pma/search.rs:7` and `fda_510k/search.rs` (both pass only `trade_name` to `device_search::page`); openFDA's PMA `ao_statement` field. Check what the 510(k) endpoint offers live before designing.

## Keeps

- Keeps: the 1.0.0 schema rules (an added optional filter or field only); trade-name search behaviour; paging totals.

## Defers

- Defers: other FDA tables; ranking.

## Changes

- Changes: the PMA and 510(k) search adapters, the diagnostic search filter, help and cases.

## Progress

- 2026-11-14, measured live: openFDA PMA `ao_statement:"cell-free"` counts 12 records (Shield P230009, SimpleScreen CRC P250029, Guardant360, FoundationOne Liquid CDx, Signatera CDx and others, supplements included); `"cell free"` also 12, `"cfDNA"` 13, `"liquid biopsy"` none, `trade_name:"cell-free"` none. openFDA answers 400 to non-ASCII letters and to `!` inside the phrase. The 510(k) endpoint has no intended-use text: its rows carry `device_name`, `statement_or_summary` (the document kind, not its text), committee, product code and decision fields.
- Changed: `search diagnostic` takes an optional `intended_use` (`--intended-use`) filter for `--source fda-pma`, sent as `ao_statement:"PHRASE"` with the device-recall phrase rules (refused punctuation spaced). It is refused with a message for every other source, including 510(k), and for non-ASCII text. PMA and 510(k) pages now carry `source_query`, the exact openFDA expression, which names the field each filter matched. `--name` still matches only `trade_name`/`device_name`. Schema change: one optional request field, two added error messages, one description.
- Proof: `tests/cases/diagnostic/intended-use.yaml` replays live replies recorded in jobs j40449 and j40467 (`tests/replies/diagnostic/intended-use/`).

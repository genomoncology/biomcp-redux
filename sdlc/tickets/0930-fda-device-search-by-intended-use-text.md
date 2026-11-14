# 0930 — fda-device-search-by-intended-use-text

Status: OPEN.

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

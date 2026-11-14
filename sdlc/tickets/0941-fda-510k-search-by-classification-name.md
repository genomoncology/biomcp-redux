# 0941 — fda-510k-search-by-classification-name

Status: OPEN.

Milestone: 7

## Outcome

`search diagnostic --source fda-510k` can match FDA's device classification name (openFDA's harmonized `openfda.device_name`, e.g. "…Tumor Profiling Test Of Circulating Cell-Free Nucleic Acids"), so a 510(k) or de novo device of a kind is one call. The reply says which field matched, as 0930's `source_query` does.

## Why

0930's review (2026-11-14): openFDA has no intended-use text for 510(k) records, so 0930 refuses `--intended-use` there and points at `--product-code`. But an agent rarely knows the product code. The dx-lab-director persona missed PGDx elio plasma focus Dx (DEN230046, product code SBY), the de novo cell-free DNA device (bench `personas/dx-lab-director/runs/2026-11-14/analysis.md`); a classification-name search for "cell-free" finds 4 records today, DEN230046 among them, while `--name` (device_name) finds blood tubes.

## Proof

- Proof: a recorded openFDA 510(k) reply for a classification-name phrase ("cell-free") returns DEN230046 and names the matched field.

## Starts from

- Starts from: 0930's `--intended-use` (PMA, `ao_statement`) and its shared `phrase()` quoting in `crates/biomcp-adapters/src/fda_device_recall/mod.rs`; `crates/biomcp-adapters/src/fda_510k/search.rs`. Decide whether this is a new filter or `--intended-use` widened to the classification name on 510(k) with the reply saying so; prefer the one that keeps the field's meaning honest.

## Keeps

- Keeps: the 1.0.0 schema rules; trade/device-name search; 0930's refusal wording if a separate filter is chosen.

## Defers

- Defers: PMA classification names.

## Changes

- Changes: the 510(k) search adapter, the diagnostic search filter, help, cases.

## Progress

- 2026-11-14, measured live: openFDA 510(k) `openfda.device_name:"cell-free"` counts 4 records: DEN230046 PGDx elio plasma focus Dx and K260827 Oncomine Dx Express Test (SBY, "High Throughput Sequencing Based Tumor Profiling Test Of Circulating Cell-Free Nucleic Acids") and two Cell-Free DNA BCT blood tubes (QMA). `"cell free"` also counts 4. `device_name:"cell-free"` counts only the 2 blood tubes.
- Design: a separate filter, `classification_name` (`--classification-name`), for `--source fda-510k` only. Widening `--intended-use` would make one flag mean the approval order's intended-use statement on PMA and the shared device-type name on 510(k); the classification name belongs to the product code, so every device under SBY matches whatever it is cleared for. A separate filter keeps both meanings honest and leaves 0930's refusal wording unchanged. `--classification` was already taken by the recall class, hence the longer name.
- Changed: the filter is sent as `openfda.device_name:"PHRASE"` with 0930's `phrase()` rules, and `source_query` shows it. It is refused with a message for any other source (PMA classification names deferred) and for non-ASCII text. Schema change: one optional request field and two added error messages.
- Proof: `tests/cases/diagnostic/classification-name.yaml` replays live replies recorded in jobs j40733 and j40739 (`tests/replies/diagnostic/classification-name/`): the classification-name search returns 4 records with DEN230046 and `source_query` `openfda.device_name:"cell-free"`; the `--name` control returns the 2 blood tubes. In j40733 the second `--record` into the same folder failed with `local.failure`; recorded alone in j40739 it succeeded.

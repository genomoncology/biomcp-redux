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

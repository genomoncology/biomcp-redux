# 1239 — pubtator-species-and-cell-line

Status: COMPLETE.
Landed: 85683decedb334f513e1651fa8648ac1f5073d45

Milestone: 10

## Outcome

`get article <PMID> --section annotations` returns PubTator's Species and CellLine annotations alongside genes, diseases, chemicals and variants, or states which entity types it leaves out.

## Why

Two of PubTator's six entity types are dropped today with no problem and no notice.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get article 19282848 --section annotations` returns gene, disease, chemical and variant entities under `status: complete`. PubTator's own export for the article also holds seven Species annotations.
- Cause, read from code: the type mapper returns nothing for unlisted types. Cell-line annotations are inferred from the same code path, not observed.

## Proof

- Proof: an outside-in case on the recorded PubTator export: the reply lists the species entities, or carries a notice naming the omitted types.

## Starts from

- Starts from: `crates/biomcp-adapters/src/pubtator/map.rs`.

## Keeps

- Keeps: the four entity types returned today are unchanged.

## Defers

- Defers: Cellosaurus search rows without the contamination warning that `get cell-line` shows, and the Discontinued flag; file when a case needs them.

## Changes

- Changes: `crates/biomcp-adapters/src/pubtator/`, schemas, article cases.

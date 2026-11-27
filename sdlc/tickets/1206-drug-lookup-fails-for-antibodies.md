# 1206 — drug-lookup-fails-for-antibodies

Status: OPEN.

Milestone: 10

## Outcome

`get drug <name>` with no `--source` resolves a drug whose top MyChem.info hit is a record the source cannot serve, by moving to the next usable hit or to another identity source, and never ends as `failed` when `--source chembl` resolves the same name.

## Why

The default lookup fails today for pembrolizumab, nivolumab, adalimumab, rituximab, bevacizumab and denosumab, so the most common oncology biologics have no default drug card.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get drug pembrolizumab` returns `status: failed` with every identity source `not-found`, and the reply's `query.input` is `187051801`. `biomcp get drug pembrolizumab --source chembl` returns `complete`.
- Cause seen from outside: MyChem.info's search for the name returns records with numeric `_id` values first (`187051801`, `377483934`, ...), and its own record endpoint answers 404 for that ID. `get drug adalimumab` and `get drug nivolumab` fail the same way with `bad-reply`.
- On a stratified sample of 41 ChEMBL molecules the default name lookup resolved 32 of 38 named ones; all 41 resolved by ChEMBL ID with `--source chembl`. Small molecules such as osimertinib resolve.

## Proof

- Proof: outside-in cases on recorded MyChem replies for pembrolizumab and adalimumab return a drug card whose identity matches the `--source chembl` card, and the reply states which hit was used.
- Proof: the drug identity census in the reach census job runs the default lookup over a ChEMBL sample stratified by molecule type and holds the resolved share as a floor.

## Starts from

- Starts from: `crates/biomcp-adapters/src/mychem/` (`names.rs`, `normalize.rs`, `map.rs`) and the drug identity resolution in `crates/biomcp-query/src/drug/`.

## Keeps

- Keeps: names that resolve today keep their identity and their stated name resolution.

## Defers

- Defers: accepting DrugBank IDs, UNII, RxCUI, CAS numbers and PubChem CIDs as `get drug` input (MyChem resolves each; the default route refuses them, and the `xrefs` section prints IDs that cannot be passed back). File separately.

## Changes

- Changes: `crates/biomcp-adapters/src/mychem/`, `crates/biomcp-query/src/drug/`, drug cases and replies.

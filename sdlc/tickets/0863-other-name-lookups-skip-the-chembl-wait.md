# 0863 — other-name-lookups-skip-the-chembl-wait

Status: OPEN.

Milestone: 7

## Outcome

Every drug-name lookup that still waits on ChEMBL's API (for example WHO prequalification search) uses 0843's MyChem copy of ChEMBL's records instead.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: pt-committee log.md:52: a WHO prequalification search timed out waiting on ChEMBL; 0843 covered trial, article and adverse-event lookups only.

## Proof

- Proof: a routine case with ChEMBL stalled shows the WHO search widened and complete.

## Starts from

- Starts from: 0843's `crates/biomcp-adapters/src/mychem/chembl_names.rs` and remaining ChEMBL-API name callers.

## Keeps

- Keeps: `get/search drug --source chembl` still use ChEMBL's API.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `tests/cases/`

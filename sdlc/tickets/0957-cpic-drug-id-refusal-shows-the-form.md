# 0957 — cpic-drug-id-refusal-shows-the-form

Status: COMPLETE.
Landed: 2582b41c8282d9530b2d399aeb3a77a167a25c14

Milestone: 7

## Outcome

`get pgx --source cpic recommendations --drug-id clopidogrel` (a plain drug name, or another wrong form) either resolves to CPIC's drug key or is refused with a message that shows the accepted form with an example (`--drug-id RxNorm:32968`), not only the generic "Checked prefix:local ontology identifier."

## Why

dx-lab-director round 3 (bench 0050, 2026-11-14, BioMCP c71f04301): calls 81, 82 and 89 were refused for `--drug-id clopidogrel` and `--drug-id PA166104521`; the agent found `RxNorm:32968` from memory. Every PGx consult starts from a drug name. BioMCP's own `--phenotype` error already names the form (`crates/biomcp-types/src/response/error.rs:172`). Evidence: bench `personas/dx-lab-director/runs/2026-11-14-r3/analysis.md`, log.md:84.

## Proof

- Proof: a case runs the plain-name call and shows either the resolved CPIC rows or a refusal naming `RxNorm:32968`-style form; it fails on main.

## Starts from

- Starts from: the CPIC recommendations validation and `error.rs:172`; whether CPIC's drug list allows resolving a name to its key (check live first; if it does, prefer resolving).

## Keeps

- Keeps: 0936's `--phenotype` behaviour and its refusal when `--drug-id` is missing.

## Defers

- Defers: nothing.

## Changes

- Changes: pgx query validation or resolution, a pgx case.

## Progress

- CPIC checked live 2026-11-14: `/v1/drug?name=ilike.clopidogrel` returns `RxNorm:32968`, and CPIC's 324 drug names are unique (305 RxNorm keys, the rest ATC, DrugBank, MeSH and MedDRA). The drug table carries `clinpgxid` (clopidogrel PA449053), not the PA166104521 the agent tried, and a brand name (Plavix) matches nothing.
- Chose the refusal over resolving. `--drug-id` is a checked CURIE that every pgx reply echoes as `query.drug_id`, so resolving a name means a schema change to the request and reply, plus an extra CPIC call that still fails brand names and spellings. The refusal path needs no schema change beyond one new error-message literal (an enum widening).
- `PgxGet` now names `drug_id` failures with `PgxDrugId`: "--drug-id takes CPIC's drug key, not a drug name, such as --drug-id RxNorm:32968 for clopidogrel; search pgx --gene <SYMBOL> --source cpic lists each drug's drugid". The same pattern as GWAS's `--p-value` message. `--phenotype` and its missing-`--drug-id` refusal are unchanged.
- Case `tests/cases/pgx/drug-id-refusal-shows-the-form.yaml` runs `--drug-id clopidogrel` and `--drug-id PA166104521`; local-only, no source replies.
- Measured: the refusal is 396 bytes (job j41026); the case caps it at 455, measured plus 15%. rustfmt changed nothing.

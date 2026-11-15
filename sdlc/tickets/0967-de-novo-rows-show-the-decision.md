# 0967 — de-novo-rows-show-the-decision

Status: OPEN.

Milestone: 7

## Outcome

A de novo device row shows the decision a reader needs: BioMCP's `de-novo-granted` status is the field an agent sees first, and openFDA's "Unknown" decision value is either labeled as openFDA's or left out where BioMCP knows better.

## Why

dx-lab-director round 4 (2026-11-15): de novo rows show openFDA's "Unknown" next to BioMCP's "de-novo-granted", and the agent reported "Unknown" (calls 33, 34; log.md:29). Evidence: bench `personas/dx-lab-director/runs/2026-11-15-r4/` (job j41109, BioMCP 95a74e246, bench 0051).

## Proof

- Proof: a recorded de novo search case shows the row's decision as granted and no bare "Unknown" an agent would read as the decision.

## Starts from

- Starts from: the 510(k)/de novo row mapping in the fda-510k adapter; round 4 calls 33-34.

## Keeps

- Keeps: the source's value stays available if the schema requires it; the 1.0.0 schema rules.

## Defers

- Defers: nothing.

## Changes

- Changes: the de novo row mapping, its case.

## Progress

- Measured live 2026-11-15 (job j41185, openFDA data updated 2026-11-02): all 492 DEN records in openFDA's 510(k) data have `decision_code` DENG, and all 492 DENG records have `decision_description` "Unknown" (DEN230046 among them). SESK, SESU, SEKD, SESD and SESP are also "Unknown" there; they get no reading, so they keep openFDA's description.
- 510(k) rows and cards now put `decision_reading` right after `k_number`, ahead of the device and decision fields, and leave out `decision_description` when a reading is present and openFDA's description is "Unknown". `decision_code` DENG stays, so openFDA's own decision stays readable. Other descriptions are unchanged. Schema change: field order and descriptions only; the field was already optional.
- Proof: `tests/cases/diagnostic/fda-510k-classification-name.yaml` (the round 4 cell-free search, recorded 2026-11-14) checks both de novo rows (DEN230046, DEN200001) read `de-novo-granted` with no `decision_description`, and both K rows keep "Substantially Equivalent"; `tests/cases/diagnostic/510k-de-novo.yaml` checks the same on the DEN170058 card and that K233537 keeps "Substantially Equivalent".

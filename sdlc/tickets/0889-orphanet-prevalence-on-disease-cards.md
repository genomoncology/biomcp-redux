# 0889 — orphanet-prevalence-on-disease-cards

Status: OPEN.

Milestone: 7

## Outcome

A disease card carries Orphanet's epidemiology: prevalence (type, value, class, geography, source, validation) and inheritance and age of onset, from Orphadata's open files, so a counselor can quote a cited prevalence to a family.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`), BioMCP had no prevalence (`incidence` is SEER cancer sites only) (log.md Q10). The code-mode agent read Orphadata's `en_product9_prev.xml` and `en_product9_ages.xml` (CC BY 4.0) and quoted Kabuki syndrome's birth prevalence 1.16/100,000 and point prevalence 3.1/100,000 with PMIDs (raw log.md:215, 247-250).

## Proof

- Proof: a recorded case where MONDO:0016512 (via Orphanet:2322) carries the prevalence rows and inheritance in the recorded Orphadata files; a disease with no Orphanet link says so.

## Starts from

- Starts from: `crates/biomcp-adapters/src/disease_sources/`; Orphadata licence CC BY 4.0, attribution required.

## Keeps

- Keeps: SEER incidence for cancer sites.

## Defers

- Defers: patient-organization links (GARD's API host does not resolve; its pages are a JavaScript app).

## Changes

- Changes: a new Orphadata adapter, `crates/biomcp-query/src/disease/`, `crates/biomcp-types/`, the sources list, `tests/cases/disease/`, `docs/`

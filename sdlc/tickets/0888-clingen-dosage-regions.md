# 0888 — clingen-dosage-regions

Status: OPEN.

Milestone: 7

## Outcome

ClinGen's region dosage curations (ISCA regions, such as the recurrent 16p11.2 deletion) can be found by cytoband, coordinates or name, each with its coordinates, haploinsufficiency and triplosensitivity scores, PMIDs, date and MONDO disease, so a microarray question has an answer.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`), `search gene --band 16p11.2` returned 245 cytoband genes and BioMCP had no region entity (log.md:111-112). The code-mode agent read ClinGen's region file and found ISCA-37400, chr16:29,638,676-30,188,531, haploinsufficiency 3, triplosensitivity 3, 2026-03-18, with PMIDs (raw log.md:85).

## Proof

- Proof: a recorded case where a 16p11.2 search returns ISCA-37400 and ISCA-37486 with the values in ClinGen's recorded region file; a coordinate search returns the regions that overlap it.

## Starts from

- Starts from: `crates/biomcp-adapters/src/clingen/` (BioMCP already syncs ClinGen's gene files).

## Keeps

- Keeps: gene dosage on the gene card.

## Defers

- Defers: a copy-number variant entity beyond ClinGen's curated regions.

## Changes

- Changes: `crates/biomcp-adapters/src/clingen/`, `crates/biomcp-query/`, `crates/biomcp-types/`, `tests/cases/`, `docs/`

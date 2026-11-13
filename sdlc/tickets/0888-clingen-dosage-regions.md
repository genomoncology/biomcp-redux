# 0888 — clingen-dosage-regions

Status: COMPLETE.
Landed: add6fad4d317491df1a2bdc5169fac41711084b3

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

## Progress

- Measured live in job j39284 (2026-11-13): the code-mode agent's file is `https://ftp.clinicalgenome.org/ClinGen_region_curation_list_GRCh38.tsv`, 98,881 bytes, `text/tab-separated-values`, 513 ISCA rows. Five `#` banner lines; the fifth names 23 columns (`#ISCA ID`, region name, `cytoBand`, `Genomic Location`, haploinsufficiency and triplosensitivity score, description, PMID1-6 and disease ID, `Date Last Evaluated`). A GRCh37 twin (99,039 bytes) sits beside it. ClinGen's terms page releases all curated content, dosage included, under CC0 1.0, as for the gene files. `search.clinicalgenome.org` has no region export (`/kb/gene-dosage/region/download` returns 500). 16p11.2 holds six rows: ISCA-37400 and ISCA-37486 with the values in the raw log, plus four gnomAD-SV population regions.
- Changed: `search gene` gains ClinGen regions. `--band` returns the regions whose band span contains the band, a new `--region chr16:START-END` (GRCh38) returns the regions that overlap the interval, and a query that is an ISCA ID returns that region. They come back under `result.regions` (at most 50, `regions_total` when more matched), each with ID, name, cytoband, location, haploinsufficiency and triplosensitivity score, description, PMIDs and disease ID, and date. `sync clingen` now syncs the region file too. `search gene --region` was a retired 0.x spelling; it is accepted again.
- Proof: `tests/cases/gene/regions.yaml` replays the region list recorded whole in job j39307. `search gene --band 16p11.2` returns ISCA-37400 and ISCA-37486 with the recorded scores, PMIDs, date and MONDO IDs; `search gene --region chr16:29600000-29700000` returns only ISCA-37400, the one region that overlaps; `search gene ISCA-37486` returns that region. The same recording backs the band cases in `gene/main` and the `sync clingen` case, whose totals now include the region list's 513 rows and 98,881 bytes.
- Also changed: `RowFormat::CommentedTsv` in biomcp-fetch reads a TSV whose `#` banner ends in the header line; `Leg::sibling` opens the file-server leg for local readiness. Search gene now names ClinGen among its sources, which makes the hosted-single first read 2,521 characters, 3 over the 0836 cap of 2,518; 0905 gives the first-read caps 5% room, which this fits. The gene help budget rose to 2,435 bytes for the new option.
- Review fixes: a band inside a curated span now finds it (`--band 15q12` finds ISCA-37478 at 15q11.2-q13.1, `--band 1p36.32` finds ISCA-37434 at 1p36.33-p36.31). Bands compare as digit sequences on one arm; a span across the centromere matches either arm from its end to the centromere; a query that prefixes either end still matches. A PMID cell holding several comma-separated PMIDs, as ISCA-37430 (Miller-Dieker) does, now yields all six. Lookup by name is by ISCA ID only: region names are not searched as words. The observed header of the region file skips its banner.
- Re-review fix: a sub-band query also finds a region labelled with the broader band (`--band 17q21.31` finds ISCA-37420, labelled 17q21.3); the reviewer's check of all 513 rows against UCSC band order gives no misses and no false positives. The across-the-centromere branch is gone: no published span crosses it.

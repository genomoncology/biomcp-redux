# 1051 — Show ClinGen gene dosage scores and supporting PubMed IDs

Status: COMPLETE.
Landed: be78d934f9104dc5fd74ab3e014f6d537dca4ea1

Milestone: 7

## Outcome

`get gene SCN2A --section clingen` and other matched genes return ClinGen’s numeric haploinsufficiency and triplosensitivity scores and the supporting PubMed IDs for each direction when the source provides them.

## Why

A counselor asking for evidence received only labels and dates in BioMCP. The detailed ClinGen gene file gave the code-mode agent both scores and their supporting papers.

## Proof

- Proof: Replay the command with the ClinGen SCN2A row. Copy loss scores 3 and lists PMIDs 22495306, 15028761, 28379373 and 29655203. Copy gain scores 0 with an empty PMID list. Preserve the source order when a cell contains two PMIDs.

## Starts from

- Starts from: The existing ClinGen bundle and gene card and the [curated gene dosage TSV](https://ftp.clinicalgenome.org/ClinGen_gene_curation_list_GRCh38.tsv). Its Gene ID is an NCBI Gene ID. Join through the verified identifier. [ClinGen attribution](https://clinicalgenome.org/docs/terms-of-use/)

## Keeps

- Keeps: Existing validity and dosage statuses, labels, dates and attribution. Missing data and an explicit zero score remain distinct.

## Defers

- Defers: Assess the effect on the counselor’s memo after the frozen comparison.

## Changes

- Changes: ClinGen bundle and gene query, gene section schema and CLI cases.

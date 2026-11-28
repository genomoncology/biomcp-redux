# 1253 — civic-drug-route-multiple-therapies-and-paging

Status: OPEN.

Milestone: 10

## Outcome

On the CIViC drug route, a name that matches more than one CIViC therapy returns the records of all of them, or says plainly which therapies matched and how to read each. A page that holds fewer rows than the total carries a next step to the next page.

## Why

Found in a run of all 100 items of the public CIViC MCP benchmark (Schimmelpfennig et al., Bioinformatics Advances 2026, vbag209) through BioMCP MCP tools only, GPT 6 Luna, 2026-11-28: 88 of 126 positive labels right. A reading of the 31 missed items found BioMCP usability faults behind 27 of them; reach is complete. CIViC holds two therapy IDs for trastuzumab deruxtecan, and the route read one. A page of 50 of 161 rows had no next step, so the model read the first page as the whole answer.

## Evidence

- Item k028: ERBB2 S310F, Lung Non-small Cell Carcinoma, Trastuzumab Deruxtecan.
- Item k049: KRAS G12C, Colorectal Cancer, Sunitinib: a page of 50 of 161 with no next step.

## Proof

- Proof: outside-in cases on recorded CIViC replies: trastuzumab deruxtecan returns rows from both therapy IDs, or names both; a 50-of-161 page has a next step whose call returns the next 50.

## Starts from

- Starts from: `crates/biomcp-adapters/src/civic/drug_*.rs`.

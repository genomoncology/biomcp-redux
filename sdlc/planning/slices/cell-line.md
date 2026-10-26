# Cell line slice

## Question bank

These questions are reformulated from published records and papers. They are retrieval questions about human health research. They do not ask BioMCP to select an experimental model, predict a treatment response or run an analysis. The accepted answers below identify the facts to preserve; the source capture will establish current values and availability.

| Case | Question | Published source | Accepted answer boundary |
| --- | --- | --- | --- |
| C01 | Which accession identifies MOLM-13 in the AML paper? | PMID 9305600; [Cellosaurus CVCL_2119](https://www.cellosaurus.org/CVCL_2119) | CVCL_2119, recommended name and source synonyms; no punctuation-based identity invention. |
| C02 | Which exact line does MV4;11 denote? | PMID 12529668; [CVCL_0064](https://www.cellosaurus.org/CVCL_0064) | Native search candidates, followed by exact CVCL_0064 card. |
| C03 | Does Cellosaurus list a FLT3 internal tandem duplication for MOLM-13? | PMID 12529668; CVCL_2119 | Native mutation description, HGNC identifier, zygosity and supporting references; no allele coordinate inferred. |
| C04 | What FLT3 variation does the source list for MV4-11? | PMID 12529668; CVCL_0064 | Independent source mutation record; preserve source zygosity and publication context. |
| C05 | What fusion does Cellosaurus list for MOLM-13? | PMID 9305600; CVCL_2119 | Native KMT2A-MLLT3 record and both gene identifiers, without collapsing a fusion into a single-gene allele. |
| C06 | Which disease identifiers accompany MOLM-13? | PMID 9305600; CVCL_2119 | Native NCIt and ORDO identifiers with their independent labels. |
| C07 | Which curated variations are recorded for OCI-AML3? | [CVCL_1844](https://www.cellosaurus.org/CVCL_1844); PMID 29805321 | Every returned variation with its original source context, including DNMT3A, NPM1 and NRAS when present. |
| C08 | What exact accession and source name does K-562 have? | [CVCL_0004](https://www.cellosaurus.org/CVCL_0004); PMID 29805321 | CVCL_0004 and K-562; a ChEMBL spelling difference does not change identity. |
| C09 | Which line does the accession CVCL_0005 identify? | [CVCL_0005](https://www.cellosaurus.org/CVCL_0005); PMID 29805321 | NB4 card, without selecting another line that carries NB4 as a synonym. |
| C10 | Why can a search for NB4 return another candidate? | [CVCL_8821](https://www.cellosaurus.org/CVCL_8821); PMID 29805321 | Preserve source names and synonyms for SJNB-4 independently; do not choose by name similarity. |
| C11 | Are HL-60 and HL-60(TB) the same accession? | [CVCL_0002](https://www.cellosaurus.org/CVCL_0002), [CVCL_A794](https://www.cellosaurus.org/CVCL_A794); PMID 29805321 | Two separate exact records; no merge from their similar names. |
| C12 | Which source identities accompany MOLM-13? | CVCL_2119; PMID 29805321 | Exact Cellosaurus cross-references for DepMap, ChEMBL, PharmacoDB and other returned resources. |
| C13 | Which Cellosaurus accession is linked to ACH-000362? | CVCL_2119; PMID 29805321 | Source cross-reference search candidates; a selected record must contain the exact requested cross-reference. |
| C14 | Which accession is linked to CHEMBL3706573? | CVCL_2119; PMID 29805321 | Exact cross-reference join to CVCL_2119, not a name join. |
| C15 | Does the MOLM-13 ChEMBL cell record carry EFO and CLO identifiers? | [ChEMBL cell CHEMBL3706573](https://www.ebi.ac.uk/chembl/explore/cell_line/CHEMBL3706573); PMID 29805321 | Preserve optional identifiers as absent when the native record omits them. |
| C16 | How many ChEMBL assays name the MOLM-13 cell record? | CHEMBL3706573; ChEMBL API documentation | Native assay page total for exact cell_chembl_id, with release metadata; no inferred activity count. |
| C17 | Can ChEMBL find K-562 by its Cellosaurus accession? | [ChEMBL cell CHEMBL3308378](https://www.ebi.ac.uk/chembl/explore/cell_line/CHEMBL3308378) | Every record returned by exact cellosaurus_id CVCL_0004; verify each returned identifier. |
| C18 | Which PharmacoDB cell identity claims MOLM-13's accession? | PMID 34850112; Cellosaurus PharmacoDB cross-reference MOLM13_950_2019 | Exact UID request, native integer ID and accession_id must match CVCL_2119. |
| C19 | Which published datasets contain MOLM-13 experiments? | PMID 34850112 | Source experiment IDs and dataset names, with count scope explicit; no sensitivity classification. |
| C20 | Which published metrics exist for an exact PharmacoDB cell and compound pair? | PMID 34850112 | Native experiment identities, dataset, compound and cell IDs, AAC, IC50, EC50, Einf, HS and DSS1 when present. No units or threshold invented. |
| C21 | How does PharmacoDB represent repeated experiments on the same pair? | PMID 34850112 | Preserve distinct experiment IDs and dataset context; never average or deduplicate by compound and line names. |
| C22 | What should a request return when a source has no cell record? | Cellosaurus and ChEMBL API contracts; technical control | Source-local empty result only after a valid reply; it does not establish that the cell line does not exist. |
| C23 | What should happen when a source fails or returns an HTML verification page? | API contracts; technical control | Failed source outcome with no fabricated empty biology; no retry through verification. |
| C24 | What source version supports a cell-line answer? | PMID 29805321; PMID 34850112; provider release endpoints | Source release when supplied; otherwise retrieval time explicitly labelled as such. |

## Investigation boundary

The investigation captures Cellosaurus, ChEMBL and PharmacoDB directly before proposing shared types. Legacy execution remains unmeasured until network containment is independently verified. Raw replies and licensed bodies stay outside Git. No product implementation belongs in this change.

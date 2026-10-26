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
| C15 | Does the MOLM-13 ChEMBL cell record carry EFO and CLO identifiers? | [ChEMBL cell CHEMBL3706573](https://www.ebi.ac.uk/chembl/explore/cell_line/CHEMBL3706573); PMID 25883136, DOI 10.1093/nar/gkv352 | Preserve optional identifiers as absent when the native record omits them. |
| C16 | How many ChEMBL assays name the MOLM-13 cell record? | CHEMBL3706573; ChEMBL API documentation | Native assay page total for exact cell_chembl_id, with release metadata; no inferred activity count. |
| C17 | Can ChEMBL find K-562 by its Cellosaurus accession? | [ChEMBL cell CHEMBL3308378](https://www.ebi.ac.uk/chembl/explore/cell_line/CHEMBL3308378) | Every record returned by exact cellosaurus_id CVCL_0004; verify each returned identifier. |
| C18 | Which PharmacoDB cell identity claims MOLM-13's accession? | PMID 34850112; Cellosaurus PharmacoDB cross-reference MOLM13_950_2019 | Exact UID request, native integer ID and accession_id must match CVCL_2119. |
| C19 | Which published datasets contain MOLM-13 experiments? | PMID 34850112 | Source experiment IDs and dataset names, with count scope explicit; no sensitivity classification. |
| C20 | Which published metrics exist for PharmacoDB cell 1248 and compound 53572? | PMID 34850112 | Native experiment identities, dataset, compound and cell IDs, AAC, IC50, EC50, Einf, HS and DSS1 when present. No units or threshold invented. |
| C21 | How does PharmacoDB represent repeated experiments on the same pair? | PMID 34850112 | Preserve distinct experiment IDs and dataset context; never average or deduplicate by compound and line names. |
| C22 | What should a request return when a source has no cell record? | Cellosaurus and ChEMBL API contracts; technical control | Source-local empty result only after a valid reply; it does not establish that the cell line does not exist. |
| C23 | What should happen when a source fails or returns an HTML verification page? | API contracts; technical control | Failed source outcome with no fabricated empty biology; no retry through verification. |
| C24 | What source version supports a cell-line answer? | PMID 29805321; PMID 34850112; provider release endpoints | Source release when supplied; otherwise retrieval time explicitly labelled as such. |

## Investigation boundary

The investigation captures Cellosaurus, ChEMBL and PharmacoDB directly before proposing shared types. Legacy execution remains unmeasured until network containment is independently verified. Raw replies and licensed bodies stay outside Git. No product implementation belongs in this change.


## Legacy accounting

The reviewed source tree is the 0.9 implementation in `biomcp`. Runtime output size, latency and warm-cache behavior remain unmeasured because this investigation has not established independent network containment for that executable. Source files and old recordings describe old behavior; they do not prove current direct-provider behavior.

| Legacy command or behavior | Proposed disposition | Reason and replacement |
| --- | --- | --- |
| `search cell-line QUERY`, `--query`/`-q`, `--limit`/`-l`, `--offset` | Keep search and paging; keep the explicit query alias | Preserve the literal source query and native candidate order. Default five headers; checked positive bounds. |
| Name normalization, removal of punctuation, a second hyphenless search | Retire by PM ruling | Text rewriting can join different lines. One unchanged native field query supplies candidates. |
| Human-first and identifier/synonym match reranking | Retire by PM ruling | Publish species and source names without introducing an ordering or declaring exact identity from normalized text. |
| `get cell-line CVCL_…` | Keep | Resolve the exact accession and verify the returned primary or explicitly supplied secondary accession. |
| Reverse get from DepMap, Cell Model Passports, ChEMBL and PharmacoDB IDs | Keep as candidate resolution | Verify the exact database and accession in the native cross-reference before returning a selected identity. Multiple records remain ambiguous. |
| `variants` | Keep bounded source variation rows | Preserve every gene member, reference, description and source zygosity; a free-text description is not a normalized allele. |
| `xrefs` | Keep bounded native cross-reference rows | Do not replace unobserved links with synthetic empty arrays for a fixed seven-source list. |
| `chembl` | Keep exact accession-linked metadata | Cell line metadata and assay count require separate requests and disclosure actions. A single section must not hide the count request. |
| `drug_response` | Keep source experiment metadata after exact UID/accession proof | No name fallback. The source must establish its own cell identity before its rows attach to a Cellosaurus record. |
| `all` | Retire by PM ruling | Named sections make independent source and output cost explicit. |
| `cell-line drug-response ID --dataset D --limit N --offset N` | Redesign after native contract capture | Old dataset filtering occurs after downloading every experiment for the line. A small rendered page does not bound source transfer. |
| `drug cell-lines NAME --cell-line ID/--dataset D --limit/--offset` and drug `cell_lines` section | Preserve prior retirement pending PM reconsideration | The drug slice settled retirement. A PharmacoDB-local compound ID cannot become a resolved drug identity through name matching. Any replacement must take exact provider identifiers and preserve this distinction. |
| `gene cell-lines SYMBOL --group GROUP --limit/--offset` | Defer pending exact cell identity proof | The gene slice moved this capability here. Its old HPA-to-Cellosaurus name join is forbidden. One bounded source contract capture is authorized; no whole expression download or local analysis. |
| Global JSON/Markdown, fresh/offline/record/replay, source disable and terms controls | Shared grammar | Do not add separate entity-specific transport or formatting options. |

The closed public-issue inventory reads every closed issue returned by the repository's full issue listing, then checks title and body for cell-line, Cellosaurus, PharmacoDB and ChEMBL references. No closed issue requests or reports a cell-line behavior. Issue [220](https://github.com/genomoncology/biomcp/issues/220) mentions an unrelated ChEMBL drug link in a missing drug-interactions report; that issue is not applicable to cell identity or screening. Internal legacy tickets 1202, 1205, 1213 and 1214 explain implementation history and are not public user issues.

## Identity and scientific boundaries

Cellosaurus owns the cell-line identity. Other sources retain independent identifiers and assertions. ChEMBL joins by its native `cellosaurus_id`; PharmacoDB must return the requested UID and the selected CVCL accession. A missing cross-reference does not authorize a name lookup fallback. Disease rows preserve NCIt and ORDO identifiers independently. A gene row must carry a source HGNC identifier before a gene pivot. A free-text mutation or fusion cannot become a normalized variant identifier. A PharmacoDB compound remains a provider-local compound unless its reply provides a separately verified exact drug identifier.

Clinical review accepted the question boundaries after the ChEMBL citation correction. Cellosaurus currently gives MV4-11's FLT3 internal tandem duplication an unspecified zygosity; PMID 12529668 reports expression of the mutated allele only. Preserve both source contexts and do not infer homozygosity. Cellosaurus describes MOLM-13 as heterozygous. Published screen metrics never establish a patient's treatment response, and their presence supplies no threshold, unit or sensitive/resistant classification.

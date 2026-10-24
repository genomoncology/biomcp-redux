# Gene slice

This file holds the gene slice's investigation and design, as `planning/slices.md` says. It covers the 0.9 accounting, the harvested examples, the source findings, the delight questions, the types, the command tree, the cases and the open product questions. Every fact below was observed live on 2026-10-23 unless it says otherwise.

## 1. How BioMCP 0.9.3 does genes

### Commands and options

| 0.9 command | Options | Sources it calls | What it returns |
| --- | --- | --- | --- |
| `search gene <q>` or `-q` | `--type`, `--chromosome`, `--region`, `--pathway`, `--go`, `--limit` 1-50, `--offset` | MyGene.info query | Rows of symbol, name, Entrez ID, GRCh38 span, UniProt, OMIM |
| `get gene <symbol> [sections]` | 16 sections plus `all` | MyGene.info and Open Targets for the default card | Card with symbol, name, Entrez, Ensembl, chromosome, GRCh38 span, OMIM, UniProt, type, summary, 5 aliases, 5 "clinical" diseases and 5 drugs |
| section `pathways` | | Reactome and KEGG | Pathway IDs and names |
| section `ontology` | | Enrichr GO libraries | Single-gene "enrichment" terms with p-values |
| section `diseases` | | Enrichr DisGeNET and OMIM libraries | Single-gene "enrichment" terms with p-values |
| section `diagnostics` | | NCBI GTR local bundle | Genetic tests naming the gene |
| section `protein` | | UniProt | Accession, name, function text, length, isoforms |
| section `go` | | QuickGO | GO terms with evidence codes |
| section `interactions` | | STRING | Partner symbols with scores |
| section `civic` | | CIViC | Assertions and evidence items |
| section `expression` | | GTEx | Median TPM by tissue |
| section `hpa` | | Human Protein Atlas | Tissue levels and subcellular location |
| section `druggability` | | DGIdb and Open Targets | Drug interactions and categories |
| section `clingen` | | ClinGen bulk files | Validity classifications and dosage |
| section `gencc` | | GenCC bundle | Submission-level validity |
| section `constraint` | | gnomAD | pLI, LOEUF, missense Z, synonymous Z |
| section `disgenet` | | DisGeNET, key required | Scored gene-disease pairs |
| section `funding` | | NIH RePORTER | Grants naming the gene |
| `gene definition`, `gene get` | | Same as `get gene` | Same card |
| `gene <symbol>` | | Same as `get gene` | Same card, through an external-subcommand fallback |
| `gene trials <symbol>` | `--limit`, `--offset`, `--source ctgov|nci` | ClinicalTrials.gov or NCI CTS | Trials whose text names the symbol |
| `gene drugs <symbol>` | `--limit`, `--offset` | MyChem and drug sources | Drugs listing the gene as target |
| `gene articles <symbol>` | `--limit`, `--offset` | PubMed and article sources | Article rows |
| `gene pathways <symbol>` | `--limit` 1-25, `--offset` | Reactome and KEGG | The whole gene card with the pathways section |
| `gene cell-lines <symbol> --group G` | `--limit` 1-100, `--offset` | HPA cell line data | nTPM for each cell line in one cancer group |
| `gene cspec <symbol>` | `--version`, `--capture-id`, `--files`, `--offset`, `--limit`, `document <capture-id>` | ClinGen CSpec registry | Variant interpretation criteria specifications |
| MCP | the shared `search` and `get` tools and a separate `gene_cspec` tool | | |

### Speed and size

Commands ran with `--no-cache` for cold timings, then again warm.

| Command | Cold | Warm | JSON | Markdown |
| --- | --- | --- | --- | --- |
| `get gene BRAF` | 0.46 s | 0.20 s | 4.3 KB | 1.6 KB |
| `get gene HER2` (alias) | 0.88 s | | 4.2 KB | |
| `get gene BRAF all` | 2.3 s | 3.0 s | 34.9 KB | 11.4 KB |
| `get gene TTN all` | 4.1 s | | 20.3 KB | 8.0 KB |
| `get gene BRAF diagnostics` | 7.2 s | | 47.0 KB | |
| `get gene BRAF clingen` | 2.1 s | | 5.5 KB | |
| `gene cell-lines FLT3 --group leukemia` | 3.1 s | | | |
| `search gene BRAF` | 0.31 s | | 0.7 KB | 0.4 KB |
| Each other single section | 0.6 to 1.8 s | | 4.5 to 12 KB | |

The warm `all` call was no faster than the cold one, so several sections skip the cache. Every JSON card carries about 1 KB of `section_outcomes` listing 16 sections as `not_requested`, plus `next_commands` and `suggestions` that repeat the same six commands.

### What 0.9 gets wrong

Each finding names the exact command and the observed result.

1. **The summary drops the cancer sentence.** `get gene BRAF` keeps only the first two sentences of the RefSeq summary. The full MyGene.info summary is 809 characters and its third sentence says V600E mutations are the most frequent cancer-causing mutations in melanoma and names other cancers. 0.9 cuts it, and it drops the "[provided by RefSeq, Aug 2017]" date.
2. **The protein section can return another species' protein for the wrong gene.** `get gene MIR21 protein` returned Q5RC51, PARN from the Sumatran orangutan, because 0.9 runs a UniProt text search for the symbol and takes the first hit. MIR21 is a non-coding RNA with no protein.
3. **One gene, one protein.** `get gene CDKN2A protein` returned only P42771 (p16INK4a). HGNC lists two reviewed proteins, P42771 and Q8N726 (p14ARF).
4. **No HGNC ID, cytoband, previous symbols or MANE transcript.** The card's `location` is the chromosome only ("7"), although MyGene.info returns `map_location` 7q34 and the HGNC ID in the same reply.
5. **Aliases are cut to five and filtered.** `get gene HER2` lists CD340, HER2, MLN 19, NEU, NGL. MyGene.info has 13 aliases, and 0.9 drops HER-2, HER-2/neu, c-ERB2 and TKR1 by rule. Previous symbols and aliases are mixed.
6. **Alias resolution is silent.** `get gene HER2`, `get gene SUR1`, `get gene MLL` and `get gene C10orf54` return ERBB2, ABCC8, KMT2A and VSIR with no field saying the input was an alias or a previous symbol. Alias inputs take two round trips (0.9 s against 0.5 s).
7. **The "clinical" drugs and diseases are unlabeled and arbitrary.** The default card's five drugs come from Open Targets, but `section_sources` credits only NCBI Gene and MyGene.info. BRAF lists belvarafenib, plixorafenib, sorafenib, regorafenib and tovorafenib and omits vemurafenib, dabrafenib and encorafenib. ERBB2 lists MM-111 and KBP5209 and omits trastuzumab.
8. **Ambiguous inputs fail without the HGNC candidates.** `get gene p16` and `get gene CAP` return not found. `get gene MLL2` returns not found with discover candidates KMT2B, KMT2D and "KMT2B gene", and does not say that MLL2 is the previous symbol of KMT2D and an alias of KMT2B.
9. **Withdrawn symbols look ambiguous.** `get gene BRCA3` returns "No exact gene match" with UMLS and NCIt candidates. HGNC says HGNC:18617 is an "Entry Withdrawn" phenotype-only record.
10. **Identifiers are refused or misread.** `get gene HGNC:1097` fails validation. `get gene ENSG00000157764` returns not found. `get gene 673` returns not found with an "ambiguous" list that includes RNU6-673P. `search gene 673` returns BRAF, so search and get disagree.
11. **Single-gene enrichment is noise.** `get gene BRAF diseases` lists "Reduced factor XIII activity" and "Benign Struma Ovarii" first, with p-values from enriching a one-gene set. `ontology` does the same with GO terms.
12. **The help example returns nothing.** `search gene -q kinase --type protein-coding --region chr7:140424943-140624564` returns zero rows. The region is BRAF's GRCh37 span, and the filter runs against GRCh38 positions.
13. **Client-side filters break totals.** `search gene -q kinase --chromosome 7 --limit 5` reports a total of 2179 and returns one row, because 0.9 filters the chromosome after fetching.
14. **Free-text search ranks noise first.** `search gene "homologous recombination"` returns SWI5, HRDAL, HROB, MCM9, MCM8, SFR1 and four LOC records. `search gene p16` ranks the pseudogene H3P10 and ARPC5 above CDKN2A.
15. **Drill-downs do not resolve aliases.** `gene drugs HER2` returns zero rows while `gene drugs BRAF` returns five. `gene trials HER2` sends "HER2" as literal text. `gene trials EGFR --source nci` returned zero rows.
16. **`gene pathways` returns the whole card.** It prints all 16 section outcomes and the identity block around the pathway list.
17. **Sections fail quietly.** `gencc` and `disgenet` were unavailable on this machine. `get gene MIR21 constraint` reports `data` with no values.
18. **Sections and subcommands do not compose.** `gene ERBB2 civic` joins the words into one symbol and fails validation.
19. **KEGG is called by default.** `pathways` and `all` call KEGG, whose terms restrict reuse.
20. **CSpec is variant interpretation, not gene identity.** `gene cspec PTEN` lists ten specification versions. The criteria are ACMG rule specifications for classifying variants, and one call printed 7.6 KB of criteria text.

What 0.9 does well: symbol input is case-insensitive (`braf` and ` BRAF ` work), previous symbols resolve (SEPT9 to SEPTIN9, KRAS2 to KRAS, FAM123B to AMER1, C11orf30 to EMSY), bad options fail fast with clear messages (`--limit 0`, `--go kinase`, unknown section `foo`), the CIViC, ClinGen and constraint sections return useful data in about a second, and `cell-lines` lists valid groups when given a bad one.

## 2. Harvested examples

Twenty examples, each from a public source. Gene identity facts were checked live against HGNC REST.

| # | Story | Question | Public source | What a good answer holds | Sources that answer |
| --- | --- | --- | --- | --- | --- |
| 1 | A clinician sees a KRAS G12C lung tumor and wants the gene card before the variant | What is KRAS: its IDs, place, aliases and transcript? | PMID 34096690 | HGNC:6407, 12p12.1, Entrez 3845, ENSG00000133703, previous symbol KRAS2, MANE Select NM_004985.5 / ENST00000311936.8 | HGNC, MyGene.info, MANE |
| 2 | A breast oncologist types HER2 | Tell me about the HER2 gene | PMID 29846122, PMID 37284804 | ERBB2, HGNC:3430, 17q12, Entrez 2064, matched as an alias | HGNC, MyGene.info, NCBI |
| 3 | An agent summarizes immunotherapy and searches PD-L1 | Which gene encodes PD-L1, and where is it? | PMID 27718847 | CD274, HGNC:17635, 9p24.1, Entrez 29126 | HGNC, MyGene.info |
| 4 | A report says MLL-rearranged infant leukemia | What is MLL now called? | PMID 18022353, PMID 37019990 | KMT2A, HGNC:7132, 11q23.3, matched as a previous symbol. Fusion partner lists come from the literature, not from a gene API | HGNC; partners from articles |
| 5 | Trial text names VISTA or C10orf54 | What gene is VISTA? | PMID 24691993 | VSIR, HGNC:30085, 10q22.1. CIViC has the gene with no variants | HGNC, CIViC |
| 6 | A myeloma panel reports FAM46C | What is FAM46C called now? | PMID 28931820 | TENT5C, HGNC:24712, 1p12, matched as a previous symbol | HGNC |
| 7 | An older lymphoma paper says MLL2 | Which gene is MLL2? | PMID 18022353 | Two candidates: KMT2D (previous symbol, HGNC:7133, 12q13.12) and KMT2B (alias, HGNC:15840, 19q13.12). No guess | HGNC |
| 8 | A pathology report says p16 loss | What gene is p16? | PMID 32747822 | Two HGNC candidates: CDKN2A (protein-coding, 9p21.3) and H3P10 (pseudogene, 3p24.2) | HGNC |
| 9 | A gene list from a spreadsheet holds SEPT9 | Look up SEPT9 | PMID 27552985, PMID 34329294 | SEPTIN9, HGNC:7323, 17q25.3, matched as a previous symbol | HGNC |
| 10 | A family asks about BRCA3 testing | What is BRCA3? | PMID 16614877 | No approved gene. HGNC:18617 is "Entry Withdrawn", phenotype only, no Entrez or Ensembl ID | HGNC |
| 11 | A urologic oncologist asks which genes select for olaparib | Which HRR genes were used in PROfound? | PMID 32343890 | BRCA1, BRCA2 and ATM in cohort A, each with its HGNC ID. The cohort B list comes from the full text and needs a check before use as a case | HGNC batch get |
| 12 | A colorectal tumor is MMR-deficient | Which genes cause Lynch syndrome, and how strong is the evidence? | PMID 25070057 | MLH1, MSH2, MSH6, PMS2, EPCAM with ClinGen validity rows | HGNC, ClinGen |
| 13 | A young woman has early breast cancer and a germline TP53 finding | Which syndrome is TP53 linked to, and how constrained is it? | PMID 32457520, PMID 32461654 | ClinGen validity rows for TP53, gnomAD v4 pLI and LOEUF with version | ClinGen, gnomAD |
| 14 | An agent must say whether NOTCH1 loss or gain matters | Is NOTCH1 an oncogene or a tumor suppressor? | PMID 30293088, PMID 28890946 | The roles as a source states them, without a pick. Only OncoKB and COSMIC carry a role field, and both have license terms | OncoKB list (license question), CIViC description |
| 15 | A pediatric sarcoma shows a possible TRK fusion | Which genes are the NTRK fusion targets? | PMID 31268127, PMID 29466156 | NTRK1, NTRK2, NTRK3 with HGNC IDs and CIViC counts | HGNC, CIViC |
| 16 | A cholangiocarcinoma has an FGFR2 fusion | Which approved drugs target FGFR2? | PMID 32203698, PMID 37953380 | Drug list with interaction type and source. This belongs to the drug slice | DGIdb (drug slice) |
| 17 | A gastric cancer is CLDN18.2-positive | What is CLDN18.2, and where is CLDN18 expressed? | PMID 37068504, PMID 25613900 | CLDN18.2 is an isoform. The gene is CLDN18, HGNC:2039, 3q22.3. Normal tissue expression from HPA | HGNC, HPA, UniProt |
| 18 | An agent groups panel results by pathway | Which genes are in RTK-RAS? | PMID 29625050 | Reactome membership. This belongs to the pathway slice | Reactome |
| 19 | An agent decides whether to suggest tumor NGS | Which cancers does ESMO recommend NGS for? | PMID 38834388 | Guideline text. No gene API answers it | Articles |
| 20 | A KRAS query showed BRAF V600E | Show only KRAS records | genomoncology/biomcp issue 113 | Every record's gene matches the resolved gene | All sources, as a regression rule |

Shapes seen across private question sets, used only as inspiration: which gene causes a syndrome, which chromosome band holds a gene, whether a gene is an oncogene or a tumor suppressor, which cancers involve a gene, and which genes confer resistance.

Gaps no free gene API fills: fusion partner lists for KMT2A, NTRK and ALK, gene-level ESCAT tiers, and a gene's role in cancer under an open license.

## 3. How the sources answer

### Per source

| Source | Call | Cold, warm | What matters | Quirks | Terms |
| --- | --- | --- | --- | --- | --- |
| HGNC REST | `/fetch/symbol/X`, `/fetch/alias_symbol/X`, `/fetch/prev_symbol/X`, `/search/symbol:X+OR+alias_symbol:X+OR+prev_symbol:X` | 0.2 to 0.5 s | hgnc_id, symbol, name, status, locus group and type, location band, alias_symbol, prev_symbol, entrez_id, ensembl_gene_id, uniprot_ids, omim_id, mane_select, gene_group | Separates aliases from previous symbols. Case-insensitive and human only, so mouse Braf maps to human BRAF. Exact fetch returns withdrawn records, so the caller checks `status`. No successor pointer on withdrawn records. 10 requests per second | CC0 |
| HGNC bulk file | `hgnc_complete_set.txt` | one download | Every field above for 47,077 records | 17 MB TSV or 33 MB JSON, last modified 2026-10-16, ETag present | CC0 |
| MyGene.info | `POST /v3/query` with `scopes=symbol,alias,retired&species=human`; `GET /v3/gene/<entrez>` | 0.3 s for 11 terms; 0.34 to 0.5 s per gene | summary (RefSeq text with date), map_location, genomic_pos (GRCh38), genomic_pos_hg19, HGNC, MIM, uniprot, type_of_gene | `GET /query?q=HER2` ignores scopes and ranks PTPN18 above ERBB2. `q=symbol:HER2` returns nothing. Always pass `species=human`. `genomic_pos` can be a list. Full TTN reply is 151 KB, 12 KB with chosen fields. Serves replaced Entrez ID 6248 without a flag. `pathway.kegg` must be dropped | Per-field source terms |
| NCBI E-utilities | `esearch db=gene term=X[sym] AND human[orgn]`, `esummary` | 0.15 to 0.21 s | summary, maplocation, otheraliases, status and currentid, genomicinfo | `[sym]` also searches aliases. Free text returns 629 IDs for HER2. Status 1 is replaced (with currentid), 2 is discontinued. Without a key, 8 parallel calls drew 7 HTTP 429s | Public domain |
| Ensembl REST | `/lookup/symbol/homo_sapiens/X`, `/lookup/id/ENSG` | 2.0 s cold, 0.4 s warm | GRCh38 span and strand, biotype, canonical transcript | Official symbols only (HER2 is a 400). `expand=1` takes 2 to 6 s and 818 KB for TTN. `xrefs/symbol` takes up to 12 s. The GRCh37 server gives a different canonical transcript | Open |
| MANE summary | `MANE.GRCh38.v1.5.summary.txt.gz` | one 1.1 MB download | Select and Plus Clinical RefSeq and Ensembl pairs per gene | 19,438 rows. HGNC's `mane_select` carries the Select pair for free | Public domain |
| UniProt REST | `uniprotkb/search?query=gene_exact:X AND organism_id:9606 AND reviewed:true` or by accession | 0.5 s | accession, protein name, function, length, subcellular location | `gene_exact` matches synonyms. One gene can have several reviewed entries (CDKN2A has two). Full TTN entry is 496 KB. Fetch by the accessions HGNC lists, never by text | CC BY 4.0 |
| CIViC GraphQL | `gene(entrezSymbol:"X")` or `genes(entrezSymbols:[...])` | 0.2 to 0.3 s | CIViC gene ID, description, variant count, link, sources | Exact official symbol only. HER2 and braf return null. VSIR exists with no variants and no description; ABCC8 is absent, so "absent" and "present but empty" differ. One evidence query returned HTTP 500 | CC0 |
| ClinGen | gene-validity and gene-dosage CSV downloads | 1.9 to 2.8 s each | validity classification, disease and MONDO ID, mode of inheritance, date, expert panel; haploinsufficiency and triplosensitivity | The API ignores its filter and returns all 3714 rows. The CSV has a 4-line banner | CC0 |
| GenCC | submissions TSV | 2.8 s, 26.5 MB | Submitter-level validity keyed by HGNC ID | No per-gene API | CC0 |
| gnomAD GraphQL | `gene(gene_symbol, reference_genome: GRCh38){gnomad_constraint{...}}` | 1.1 s cold, 0.13 s warm | pLI, LOEUF (oe_lof_upper), mis_z, flags, transcript | Official symbol only. GRCh37 serves v2 constraint with different values (BRCA2 LOEUF 0.635 against 0.824) | CC0 |
| Human Protein Atlas | `/ENSG….json` | 0.5 s | tissue and cancer specificity, subcellular location, protein class | Text search is fuzzy (BRAF also returns HMG20B), so fetch by Ensembl ID | CC BY-SA 4.0 |
| Reactome | `/ContentService/data/mapping/UniProt/<acc>/pathways?species=9606` | 0.8 s | lowest-level pathway IDs and names | Needs the UniProt accession first | CC BY 4.0 |
| OncoKB | `/api/v1/utils/cancerGeneList` without a token | 0.26 s, 1247 rows | geneType, panel flags, chosen transcript per build | Public endpoint, but OncoKB terms allow free academic research use only. `allCuratedGenes` also carries summary text | Restricted |
| DGIdb GraphQL | `genes(names:[...])`, `geneMatches` | 0.5 s | drug interactions and sources; ambiguity labels | Mixed source licenses inside the aggregate. Belongs to the drug slice | Mixed |

### Where sources disagree

- **Aliases.** HGNC lists 8 ERBB2 aliases and one previous symbol. MyGene.info lists 13 aliases. NCBI maps AIB1 to NCOA3 and ANIB1, while HGNC maps it to NCOA3 only.
- **Previous symbol against alias.** MLL is only a previous symbol in HGNC, so an alias-only lookup fails. MLL2 is a previous symbol of KMT2D and an alias of KMT2B.
- **Ambiguity counts.** p40 matches 10 genes in HGNC, 14 in NCBI, 11 in MyGene.info and 7 in DGIdb. HGNC includes TP63 among them.
- **Transcript versions.** HGNC MANE Select for BRAF is NM_004333.6. OncoKB's list names NM_004333.4 with the same Ensembl transcript ENST00000646891. KRAS shows NM_004985.5 against NM_004985.3.
- **Coordinates.** MyGene.info gives BRAF 7:140719327-140925199 on GRCh38 and a GRCh37 start of 140419127. Ensembl GRCh37 gives 7:140419127-140624564 with a different canonical transcript. NCBI `chrstart` is zero-based.
- **Constraint.** gnomAD v4 on GRCh38 and v2 on GRCh37 give different values for the same gene.

### Which source owns which field

| Field | Owner | Why |
| --- | --- | --- |
| HGNC ID, approved symbol, name, status | HGNC | Naming authority. Shows withdrawn records. CC0 |
| Previous symbols and aliases, kept apart | HGNC | Only HGNC separates them, and MLL needs it |
| Locus group, locus type, cytoband, gene groups | HGNC | Curated, one record |
| Entrez, Ensembl, UniProt, OMIM IDs | HGNC | One record holds them all |
| MANE Select pair | HGNC `mane_select` | Already in the record. The MANE file adds Plus Clinical |
| RefSeq summary with its date | MyGene.info (same text as NCBI) | One fast call, no 3 per second limit |
| GRCh38 and GRCh37 spans with strand | MyGene.info | Both builds in the same call as the summary. Ensembl lookup is the slower fallback |
| Replaced or discontinued Entrez IDs | NCBI esummary | MyGene.info is stale on replaced IDs |
| Protein name, function, length, location | UniProt by HGNC's accessions | Avoids the MIR21 failure. Covers genes with several proteins |
| Open cancer evidence summary and variant count | CIViC | CC0. Takes the HGNC symbol |
| Gene-disease validity and dosage | ClinGen bulk files | CC0. Daily files |
| Loss-of-function constraint | gnomAD v4, GRCh38 | Current release. Version is stated |
| Normal tissue expression and location | HPA by Ensembl ID | CLDN18 case needs it |
| Pathway membership | Reactome by UniProt accession | Open. KEGG excluded |

### Requests and speed for the design below

- Default card: one HGNC lookup and one MyGene.info gene call, in parallel once the HGNC record gives the Entrez ID. With a local HGNC bundle the HGNC part takes no request, and the card is one MyGene.info call of about 0.35 s cold.
- Each section adds one request that runs in parallel: UniProt 0.5 s, CIViC 0.3 s, gnomAD 1.1 s cold, HPA 0.5 s, Reactome 0.8 s after UniProt. ClinGen reads a local bundle.
- `get gene X all` should finish in about 1.5 s cold, set by gnomAD and Reactome, against 2.3 to 3.0 s in 0.9.

## 4. Delight questions

1. **What does the agent ask first, and which command answers it?** "What is this gene?" given whatever the clinician typed: a symbol, an alias, an old symbol or an ID. `get gene <text>` answers it. `search gene <text>` answers "which genes match this word or band".
2. **What is the smallest header that lets the agent choose its next step?** HGNC ID, symbol, name, locus group, cytoband, status, and how the input matched: symbol, previous symbol, alias, or ambiguous. These come from one HGNC record.
3. **Which drill-downs does the agent need next, and does `next` name each one?** The card's `next` names the sections with data behind them, chosen from the identity record: `protein` only when HGNC lists UniProt accessions, `civic` only when CIViC has the gene. Pivots to trials, articles, variants and drugs appear in `next` once those slices land, as `search trial --gene ERBB2` and similar.
4. **Which identifiers must come back exact?** The HGNC ID and approved symbol for every later command. The Entrez ID for PubMed and ClinVar. The Ensembl gene ID for HPA and gnomAD. The UniProt accessions for Reactome and the protein slice. The MANE Select RefSeq and Ensembl transcripts with versions for variant naming.
5. **Where do sources disagree, and how does the output show it?** Aliases keep their source: HGNC aliases and previous symbols as the main lists, NCBI-only aliases in a separate list. Transcripts carry versions. Coordinates carry their build and counting rule. The output never merges or picks between two sources' values.
6. **What can fail or come back empty, and how does the status tell them apart?** Input: resolved, ambiguous, withdrawn, or not found. Section: data, confirmed empty (CIViC has VSIR with no variants), not applicable (no protein for MIR21), or unavailable (source error, with the reason). A source error never becomes an empty list.
7. **How fast is a cold and a warm call?** Cold default card about 0.35 s with a bundle and 0.5 s without, set by MyGene.info. Warm card under 50 ms, with no request.
8. **What does 0.9 return that no example needs, and what does it miss?** It returns Enrichr p-values, GO terms, STRING partners, NIH grants, GTR tests, unlabeled Open Targets drugs and diseases, and repeated next commands. It misses the HGNC ID, the cytoband, previous symbols, the MANE transcript, the full summary, every protein of a gene, how an alias matched, and withdrawn status.
9. **What lets the agent stop early with the right answer?** A card that says "HER2 matched the alias of ERBB2 (HGNC:3430)" or "MLL2 matches two genes" or "BRCA3 is a withdrawn HGNC entry, not a gene" in the first lines.
10. **What would a clinician need to trust the answer?** Each section names its source, its release or fetched date (HGNC last modified, RefSeq summary date, gnomAD version, ClinGen review date), and a link to the source record.

## 5. Design

### biodata types

Identifier newtypes, each with a checked constructor, `FromStr` and `Display`:

| Type | Text form | Rule |
| --- | --- | --- |
| `HgncId` | `HGNC:1097` | Prefix `HGNC:` and a nonzero number. Parses bare `1097` only where the caller says the namespace is HGNC |
| `EntrezGeneId` | `673` | `NonZeroU32` |
| `EnsemblGeneId` | `ENSG00000157764` | `ENSG` and 11 digits. A version suffix is parsed and kept apart |
| `GeneSymbol` | `BRAF`, `C10orf54`, `HLA-A` | HGNC symbol characters. Compares and hashes without case |
| `UniProtAcc` | `P15056` | UniProt accession pattern |
| `OmimId` | `164757` | Six digits |
| `RefSeqTranscript` | `NM_004333.6` | Prefix, number and version |
| `EnsemblTranscript` | `ENST00000646891.2` | Number and version |
| `Cytoband` | `7q34`, `9p21.3` | Chromosome, arm and band |

Closed sets: `GeneStatus` (approved, withdrawn), `LocusGroup` (protein coding, non-coding RNA, pseudogene, other, phenotype; non-exhaustive), `MatchKind` (symbol, previous symbol, alias, HGNC ID, Entrez ID, Ensembl ID).

`GeneInput` is free text that the edge parses into one of: an `HgncId`, an `EnsemblGeneId`, an `EntrezGeneId` (digits only), or a symbol-like text. Alias text stays text because aliases hold spaces, slashes and parentheses ("HER-2/neu", "MLN 19").

`Gene` (schema `biodata.gene/1`), built from HGNC with MyGene.info fields added:

- `hgnc_id: HgncId`, `symbol: GeneSymbol`, `name`, `status: GeneStatus`, `locus_group: LocusGroup`, `locus_type`
- `location: Cytoband` (absent for genes with no band)
- `previous_symbols: [GeneSymbol]`, `aliases: [text]`, `ncbi_aliases: [text]` (the NCBI aliases HGNC lacks)
- `entrez_id`, `ensembl_id`, `uniprot: [UniProtAcc]`, `omim: [OmimId]`
- `mane_select: { refseq: RefSeqTranscript, ensembl: EnsemblTranscript }`
- `spans: [GenomicInterval]`, one per build, each with strand
- `summary: { text, source: refseq, dated }` (whole text, never cut)

`GeneHeader` (schema `biomcp.gene_header/1`): `hgnc_id`, `symbol`, `name`, `locus_group`, `location`, `status`, and `match: { kind, text }`.

`GeneResolution`: `resolved { gene, match }`, `ambiguous { candidates: [GeneHeader] }`, `withdrawn { hgnc_id, symbol, locus_type }`, or `not_found`.

### Summary and sections

The default card is the identity record, the spans, the MANE Select pair and the full RefSeq summary. It is about 1.5 KB of JSON for BRAF.

Sections, a closed enum:

| Section | Source | Holds | Not applicable when |
| --- | --- | --- | --- |
| `protein` | UniProt, by HGNC accessions | per protein: accession, name, length, function text, subcellular location | HGNC lists no UniProt accession |
| `civic` | CIViC | CIViC gene ID, description, variant count, link | never; absent and empty are both answers |
| `clingen` | ClinGen bundle | validity rows (disease, MONDO ID, inheritance, classification, date, panel), haploinsufficiency, triplosensitivity | never |
| `constraint` | gnomAD v4 GRCh38 | pLI, LOEUF, mis_z, transcript, version | locus group is not protein coding |
| `expression` | HPA | RNA tissue specificity, top tissues, subcellular location, release | HPA has no record for the Ensembl ID |
| `pathways` | Reactome | pathway ID and name | no UniProt accession |

`all` requests every section. Requested sections report one of the four states. Unrequested sections are absent.

### Command tree

| Command | Returns | Notes |
| --- | --- | --- |
| `biomcp search gene <text>` | `GeneHeader` page, ranked: exact symbol, previous symbol, alias, then name words; protein coding before other locus groups | Ambiguous aliases come back as several headers, each with its `match` |
| `search gene --band 9p21.3` | Headers for genes in a cytoband | From HGNC `location` in the bundle; no live API filters by band. Answers "what else sits in this deleted band" (CDKN2A, CDKN2B, MTAP) |
| `search gene --locus-group protein-coding` | Filter | Replaces 0.9 `--type` |
| `biomcp get gene <input>...` | One card per input, with its `GeneResolution` | Accepts symbols, previous symbols, aliases, `HGNC:` IDs, Entrez IDs and Ensembl IDs. Several inputs at once (HRR and Lynch panels) |
| `get gene <input> <section>...` | Card plus sections | `protein`, `civic`, `clingen`, `constraint`, `expression`, `pathways`, `all` |

`next` on a resolved card names each section with data behind it. On an ambiguous result, it names `get gene <hgnc_id>` for each candidate. On a header page, it names `get gene` for the first header.

### Keep, change or drop each 0.9 feature

| 0.9 feature | Ruling | Reason |
| --- | --- | --- |
| `search gene <q>` | Keep, change | Rank by match kind from HGNC. Return HGNC ID, band, locus group and match kind. Examples 7 and 8 need ranked candidates |
| `--type` | Rename `--locus-group` | HGNC's closed set replaces MyGene.info's free text |
| `--chromosome` | Drop | `--band` covers it with a real case. 0.9's client-side filter broke totals |
| `--region` | Drop | Coordinate search belongs to the variant slice's copy-number questions. 0.9's help example returns nothing |
| `--pathway`, `--go` | Drop | No harvested example. Pathway membership belongs to the pathway slice |
| `--limit`, `--offset` | Change | The shared cursor from `architecture.md` |
| `get gene <symbol>` | Keep, change | Add IDs as input, report how the input matched, return candidates for ambiguous input and status for withdrawn records |
| Default card fields | Change | Add HGNC ID, band, previous symbols, MANE, both builds, full summary. Drop the Open Targets drugs and diseases, which are unlabeled and miss the approved drugs |
| `protein` | Keep, change | Fetch by HGNC accessions. Return every reviewed protein |
| `civic` | Keep, change | Gene description, variant count and link in the card. Assertions and evidence move to the variant slice |
| `clingen` | Keep | Examples 12 and 13 |
| `constraint` | Keep | Example 13 |
| `hpa` | Rename `expression` | Example 17. One expression source |
| `expression` (GTEx) | Drop | HPA covers the example. Two expression sources repeat one idea |
| `pathways` | Keep, change | Reactome only. KEGG dropped |
| `ontology`, `diseases` | Drop | Single-gene enrichment p-values are noise |
| `go` | Drop | No example. MyGene.info carries GO if a later case needs it |
| `interactions` | Drop | No example. Bench biology |
| `druggability` | Move to the drug slice | Example 16 asks a drug question |
| `gencc` | Defer | ClinGen answers examples 12 and 13. GenCC returns when a case finds a gene ClinGen has not curated. 26.5 MB bundle |
| `disgenet` | Drop | Needs a key, has license terms, and was unavailable. ClinGen covers validated gene-disease links |
| `diagnostics` | Move to the diagnostic slice | Genetic tests are their own entity. 7.2 s and 47 KB for BRAF |
| `funding` | Drop | Not a cancer care question |
| `gene definition`, `gene get`, `gene <symbol>` | Drop | One spelling per idea |
| `gene trials` | Move to the trial slice | It becomes `search trial --gene`, fed the resolved symbol and aliases |
| `gene articles` | Move to the article slice | It becomes `search article --gene` |
| `gene drugs` | Move to the drug slice | It becomes `search drug --target` |
| `gene pathways` | Fold into the `pathways` section | 0.9 printed the whole card anyway |
| `gene cell-lines` | Defer to the cell line slice | Cell line expression is bench research |
| `gene cspec` and MCP `gene_cspec` | Move to the variant slice | Variant interpretation criteria |

### Sources kept and dropped

Kept: HGNC (bundle or REST), MyGene.info, UniProt, CIViC, ClinGen bundle, gnomAD, HPA, Reactome. NCBI E-utilities stays as the check for replaced Entrez IDs.

Dropped from the gene slice: Open Targets, Enrichr, QuickGO, STRING, GTEx, DisGeNET, NIH RePORTER, GTR, KEGG, CSpec, DGIdb (to the drug slice), GenCC (deferred), OncoKB (pending Ian's license ruling).

## 6. Outside-in cases

Each case was verified on 2026-10-23 against live HGNC REST, MyGene.info or CIViC. Each asserts stable identity facts, ordering rules and states, not free text that drifts.

| # | Story | Run | Expect |
| --- | --- | --- | --- |
| 1 | KRAS G12C lung tumor, gene card first (PMID 34096690) | `get gene KRAS` | `/status` complete; `hgnc_id` HGNC:6407; `entrez_id` 3845; `ensembl_id` ENSG00000133703; `location` 12p12.1; `previous_symbols` includes KRAS2; `mane_select.refseq` starts NM_004985.; `match.kind` symbol |
| 2 | Oncologist types HER2 (PMID 29846122) | `get gene HER2` | `symbol` ERBB2; `hgnc_id` HGNC:3430; `location` 17q12; `match.kind` alias; `match.text` HER2 |
| 3 | MLL-rearranged leukemia (PMID 18022353) | `get gene MLL` | `symbol` KMT2A; `hgnc_id` HGNC:7132; `match.kind` previous_symbol |
| 4 | Myeloma panel says FAM46C (PMID 28931820) | `get gene FAM46C` | `symbol` TENT5C; `hgnc_id` HGNC:24712; `location` 1p12; `match.kind` previous_symbol |
| 5 | Old paper says MLL2 (PMID 18022353) | `get gene MLL2` | resolution ambiguous; candidates exactly {HGNC:7133, HGNC:15840}; KMT2D (previous symbol) ranks before KMT2B (alias); `next` holds `get gene HGNC:7133` and `get gene HGNC:15840` |
| 6 | Pathology says p16 loss (PMID 32747822) | `get gene p16` | resolution ambiguous; candidates include HGNC:1787 and HGNC:43802; CDKN2A (protein coding) ranks first |
| 7 | Family asks about BRCA3 (PMID 16614877) | `get gene BRCA3` | resolution withdrawn; `hgnc_id` HGNC:18617; no card; never BRCA1 or BRCA2 |
| 8 | Nonsense input | `get gene NOTAGENE` | resolution not_found; error code `not_found`; the input text never appears in the message |
| 9 | Agent holds an ID from another tool | `get gene HGNC:1097`, `get gene 673`, `get gene ENSG00000157764` | each returns `symbol` BRAF with `match.kind` hgnc_id, entrez_id, ensembl_id |
| 10 | BRAF summary keeps the cancer sentence (0.9 finding 1) | `get gene BRAF` | `summary.text` includes "V600E"; `summary.dated` present |
| 11 | CDKN2A encodes two proteins (0.9 finding 3) | `get gene CDKN2A protein` | `protein` state data; accessions exactly {P42771, Q8N726} |
| 12 | MIR21 has no protein (0.9 finding 2) | `get gene MIR21 protein constraint` | `locus_group` non_coding_rna; `protein` state not_applicable; `constraint` state not_applicable; no accession anywhere |
| 13 | VISTA in trial text (PMID 24691993) | `get gene VISTA civic` | `symbol` VSIR; `civic` state confirmed empty with CIViC gene ID 14308 |
| 14 | Lynch panel (PMID 25070057) | `get gene MLH1 MSH2 MSH6 PMS2 EPCAM clingen` | five cards in input order; each `clingen` state data |
| 15 | 9p21 deletion, which genes go with CDKN2A | `search gene --band 9p21.3` | headers include CDKN2A, CDKN2B and MTAP |
| 16 | CIViC fails during a card | `get gene ERBB2 civic` with a recorded HTTP 500 | `/status` partial; `civic` state unavailable with reason; identity fields still present |

Cases 5, 6 and 15 assert sets and order rules that BioMCP owns, so upstream additions do not break them. HGNC REST fetch gives CDKN2A, CDKN2B and MTAP each the location 9p21.3. Neither HGNC REST search nor MyGene.info can query by band (MyGene.info answers "map_location is not indexed"), so case 15 and `--band` need the HGNC bundle from question 1.

## 7. Questions for Ian

1. **Should HGNC ship as a local bundle?** Recommendation: yes. The CC0 file is 17 MB, changes about weekly, and has an ETag. It makes resolution and `search gene` instant and offline, gives deterministic ambiguity and withdrawn answers, saves one request on every gene, variant and article call that names a gene, and is the only way to offer `search gene --band`, since no live API filters by band. The cost is a `biomcp sync hgnc` step and 17 MB of disk. Without it, every gene input costs one 0.2 to 0.5 s HGNC call.
2. **When an alias matches several genes, should BioMCP pick one?** Recommendation: never pick. Return ranked candidates with how each matched. MLL2 means KMT2D in older papers and is an alias of KMT2B in HGNC, and p16 matches CDKN2A and a pseudogene. A wrong silent pick would send the agent to the wrong gene, which is what issue 113 reported.
3. **May BioMCP show OncoKB's public gene list fields, such as oncogene or tumor suppressor?** Recommendation: leave OncoKB out until you rule on its license. The endpoint needs no token, but OncoKB terms limit free use to academic research. No open source carries a gene's role in cancer, so example 14 stays answered by CIViC's description and articles until you decide.
4. **Should the gene-scoped pivots (`gene trials`, `gene articles`, `gene drugs`) stay as gene commands?** Recommendation: move each to its own entity's search (`search trial --gene ERBB2`), and have the gene card's `next` name them. `architecture.md` uses `gene trials BRAF` as an example, so this needs your word. One spelling per idea favors the entity search, and 0.9's `gene drugs HER2` returning nothing shows the cost of two paths.
5. **Do you accept the drops from 0.9?** Enrichr ontology and diseases, GO, STRING, GTEx, DisGeNET, NIH funding, the Open Targets drug and disease lists, `--chromosome`, `--region`, `--pathway`, `--go` and the command aliases. Recommendation: drop them. No harvested example needs them, several return noise or wrong answers, and the parity bar covers useful behavior only.
6. **Should the default card carry the full RefSeq summary?** Recommendation: yes, whole and dated. It is about 800 characters for BRAF and holds the sentence that names V600E and the cancers. 0.9's two-sentence cut removed it.
7. **Should the card give both GRCh38 and GRCh37 spans?** Recommendation: yes. MyGene.info returns both in the same call at no cost, many lab reports and OncoKB's list still carry GRCh37 fields, and each span states its build.
8. **Should BioMCP serve non-human genes?** Recommendation: human only. Every example is human. HGNC maps mouse Braf to human BRAF, and the card should say the input matched a human symbol without regard to case.

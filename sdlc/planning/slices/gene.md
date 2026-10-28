# Gene slice

This file holds the gene slice's investigation and design, as `planning/slices.md` says. It covers the 0.9 accounting, the harvested examples, the source findings, the delight questions, the types, the command tree, the progressive disclosure layers and token budget, the keyed sources, the cases and Ian's rulings. Every fact below was observed live on 2026-10-23 or 2026-10-24 unless it says otherwise.

Ian approved all eight recommendations in section 9, as `decisions/2026-10-24-gene-slice-rulings.md` records. He then allowed OncoKB as a keyed source, as `decisions/2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` says, and section 7 adds it.

## 1. How BioMCP 0.9.3 does genes

### Commands and options

| 0.9 command | Options | Sources it calls | What it returns | 1.0 |
| --- | --- | --- | --- | --- |
| `search gene <q>` or `-q` | `--type`, `--chromosome`, `--region`, `--pathway`, `--go`, `--limit` 1-50, `--offset` | MyGene.info query | Rows of symbol, name, Entrez ID, GRCh38 span, UniProt, OMIM | `search gene <q>`; `-q` is removed. `--type` → `--locus-group` (`use: --locus-group`); `--chromosome` → `--band` (`use: --band`); `--region`, `--pathway`, `--go` dropped (`dropped in 1.0`): no harvested question needs them. `--limit` and `--offset` remain. |
| `get gene <symbol> [sections]` | 16 sections plus `all` | MyGene.info and Open Targets for the default card | Card with symbol, name, Entrez, Ensembl, chromosome, GRCh38 span, OMIM, UniProt, type, summary, 5 aliases, 5 "clinical" diseases and 5 drugs | `get gene <input>... [sections]`; `all` requests the shipped sections. `--section WORD` preserves inputs that equal section words. The card uses HGNC identity and MyGene extras; arbitrary Open Targets diseases and drugs are dropped. |
| section `pathways` | | Reactome and KEGG | Pathway IDs and names | `get gene <input> pathways`; Reactome only; KEGG dropped for its reuse terms. |
| section `ontology` | | Enrichr GO libraries | Single-gene "enrichment" terms with p-values | Dropped: single-gene enrichment does not answer a harvested question. `dropped in 1.0`. |
| section `diseases` | | Enrichr DisGeNET and OMIM libraries | Single-gene "enrichment" terms with p-values | Dropped: single-gene enrichment does not answer a harvested question. `dropped in 1.0`. |
| section `diagnostics` | | NCBI GTR local bundle | Genetic tests naming the gene | Diagnostic slice. `moved to the diagnostic entity`. |
| section `protein` | | UniProt | Accession, name, function text, length, isoforms | `get gene <input> protein`; all reviewed human proteins joined by IDs. |
| section `go` | | QuickGO | GO terms with evidence codes | Dropped: no harvested question needs GO lists. `dropped in 1.0`. |
| section `interactions` | | STRING | Partner symbols with scores | Dropped: no harvested question needs interaction scores. `dropped in 1.0`. |
| section `civic` | | CIViC | Assertions and evidence items | `get gene <input> civic`; gene description and counts; variant evidence moves to the variant slice. |
| section `expression` | | GTEx | Median TPM by tissue | GTEx is dropped because no harvested question needs it. The shipped `expression` word requests HPA. |
| section `hpa` | | Human Protein Atlas | Tissue levels and subcellular location | `get gene <input> expression`; `use: expression`. |
| section `druggability` | | DGIdb and Open Targets | Drug interactions and categories | Drug slice. `moved to the drug entity`. |
| section `clingen` | | ClinGen bulk files | Validity classifications and dosage | `get gene <input> clingen`; validity and dosage. |
| section `gencc` | | GenCC bundle | Submission-level validity | `get gene <input> gencc` reads stored submissions by HGNC identity. `sync gencc` installs or refreshes the complete CSV bundle. |
| section `constraint` | | gnomAD | pLI, LOEUF, missense Z, synonymous Z | `get gene <input> constraint`; population metrics with version. |
| section `disgenet` | | DisGeNET, key required | Scored gene-disease pairs | Dropped: ClinGen covers validated links. `dropped in 1.0`. |
| section `funding` | | NIH RePORTER | Grants naming the gene | Dropped: no harvested question needs grants. `dropped in 1.0`. |
| `gene definition`, `gene get` | | Same as `get gene` | Same card | `get gene <input>`; both aliases return `use: get gene <gene>`. |
| `gene <symbol>` | | Same as `get gene` | Same card, through an external-subcommand fallback | `get gene <input>`; fallback returns `use: get gene <gene>`. |
| `gene trials <symbol>` | `--limit`, `--offset`, `--source ctgov\|nci` | ClinicalTrials.gov or NCI CTS | Trials whose text names the symbol | Trial slice: `search trial --gene <gene>`; `use: search trial --gene <gene>`. Its paging and source options belong to that slice. |
| `gene drugs <symbol>` | `--limit`, `--offset` | MyChem and drug sources | Drugs listing the gene as target | Dropped under the 2026-10-26 pathway ruling on unsupported reverse gene-to-drug search. Select a drug identity and request explicit source sections. |
| `gene articles <symbol>` | `--limit`, `--offset` | PubMed and article sources | Article rows | Article slice: `search article --gene <gene>`; `use: search article --gene <gene>`. Its paging options belong to that slice. |
| `gene pathways <symbol>` | `--limit` 1-25, `--offset` | Reactome and KEGG | The whole gene card with the pathways section | `get gene <input> pathways`; `use: get gene <gene> pathways`. Old `--limit` and `--offset` belong to pathway paging in the pathway slice; this section returns its bounded list. |
| `gene cell-lines <symbol> --group G` | `--limit` 1-100, `--offset` | HPA cell line data | nTPM for each cell line in one cancer group | Cell line slice, including `--group`, `--limit` and `--offset`. `moved to the cell line entity`. |
| `gene cspec <symbol>` | `--version`, `--capture-id`, `--files`, `--offset`, `--limit`, `document <capture-id>` | ClinGen CSpec registry | Variant interpretation criteria specifications | Variant slice, including `--version`, `--capture-id`, `--files`, `--offset`, `--limit` and `document`. `moved to the variant entity`. |
| MCP | the shared `search` and `get` tools and a separate `gene_cspec` tool | | | One `biomcp` tool takes the same argument array as the CLI. Separate `search`, `get` and `gene_cspec` tools are replaced. |

### Leaf-command parity accounting

The historical inventory in the architecture investigation, section 6.1, counts leaf commands and command-specific option slots. Short aliases share the long option's slot. Command aliases, the external-symbol fallback, positional arguments, section names, MCP tools and global flags are outside that denominator. `gene cspec document` is the leaf; its parent's five options are outside the historical leaf inventory. This table owns `gene articles`, so article accounting does not count that command again. The variant slice owns the CSpec document leaf; each other leaf has its disposition below.

<!-- parity-accounting -->
| 0.9 leaf command | Command outcome | Ported or renamed options | Dropped options | Deferred options | Undecided options | Ruling |
| --- | --- | --- | --- | --- | --- | --- |
| `search gene` | Ported or renamed | `--type`, `--chromosome`, `--limit`, `--offset` | `--query`, `--region`, `--pathway`, `--go` | — | — | Section 1 maps type and chromosome, preserves paging and removes the query alias and unused filters. |
| `get gene` | Ported or renamed | — | — | — | — | Section 1 preserves the gene card; section values do not count as option slots. |
| `gene definition` | Ported or renamed | — | — | — | — | Section 1 replaces this leaf and its get alias with get gene. |
| `gene trials` | Ported or renamed | `--limit`, `--offset`, `--source` | — | — | — | The accepted trial rulings move discovery to search trial --gene, preserve the limit and source choices, and rename source paging to --next-page. |
| `gene drugs` | Dropped by ruling | — | `--limit`, `--offset` | — | — | The 2026-10-26 protein, pathway and study rulings retire reverse gene-to-drug search. Use a selected drug identity and its explicit source sections; no reverse search or paging replacement is approved. |
| `gene articles` | Ported or renamed | `--limit`, `--offset` | — | — | — | Section 1 maps this capability to search article --gene; article search preserves paging. |
| `gene pathways` | Ported or renamed | `--limit`, `--offset` | — | — | — | Use get gene INPUT pathways for the bounded card section; search pathway --gene INPUT --limit N --offset N owns full membership paging. |
| `gene cell-lines` | Deferred by ruling | — | — | `--group`, `--limit`, `--offset` | — | The cell line slice defers HPA expression until an exact source cell identity join exists; name matching is forbidden. |
<!-- /parity-accounting -->

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
8. **Ambiguous inputs fail without the HGNC candidates.** `get gene p16` and `get gene CAP` returned a bare not found with `--no-cache`. A later warm `get gene p16` returned not found with three UMLS candidates ("CDKN2A gene", "CDKN2A wt Allele", "H3P10 gene"), so the answer depends on cache state. `get gene MLL2` returns not found with discover candidates KMT2B, KMT2D and "KMT2B gene", and does not say that MLL2 is the previous symbol of KMT2D and an alias of KMT2B.
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
| 14 | An agent must say whether NOTCH1 loss or gain matters | Is NOTCH1 an oncogene or a tumor suppressor? | PMID 30293088, PMID 28890946 | The roles as a source states them, without a pick. Only OncoKB and COSMIC carry a role field, and both have license terms | OncoKB with the user's key, CIViC description |
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
| HGNC bulk files | `hgnc_complete_set.txt` and `withdrawn.txt` | one download each | Every field above for approved records; withdrawn symbols with their successors | The complete set is 17 MB TSV or 33 MB JSON, last modified 2026-10-16, ETag present. It leaves out withdrawn records, so BRCA3 is missing from it. `withdrawn.txt` (259 KB) lists 1,802 "Entry Withdrawn" and 3,489 "Merged/Split" symbols, the latter with the HGNC ID and symbol they merged into (A2MR into HGNC:6692 LRP1). HGNC REST cannot filter by band, and the file can | CC0 |
| MyGene.info | `POST /v3/query` with `scopes=symbol,alias,retired&species=human`; `GET /v3/gene/<entrez>` | 0.3 s for 11 terms; 0.34 to 0.5 s per gene | summary (RefSeq text with date), map_location, genomic_pos (GRCh38), genomic_pos_hg19, HGNC, MIM, uniprot, type_of_gene | `GET /query?q=HER2` ignores scopes and ranks PTPN18 above ERBB2. `q=symbol:HER2` returns nothing. Always pass `species=human`. `genomic_pos` can be a list. Full TTN reply is 151 KB, 12 KB with chosen fields. Serves replaced Entrez ID 6248 without a flag. `pathway.kegg` must be dropped | Per-field source terms |
| NCBI E-utilities | `esearch db=gene term=X[sym] AND human[orgn]`, `esummary` | 0.15 to 0.21 s | summary, maplocation, otheraliases, status and currentid, genomicinfo | `[sym]` also searches aliases. Free text returns 629 IDs for HER2. Status 1 is replaced (with currentid), 2 is discontinued. Without a key, 8 parallel calls drew 7 HTTP 429s | Public domain |
| Ensembl REST | `/lookup/symbol/homo_sapiens/X`, `/lookup/id/ENSG` | 2.0 s cold, 0.4 s warm | GRCh38 span and strand, biotype, canonical transcript | Official symbols only (HER2 is a 400). `expand=1` takes 2 to 6 s and 818 KB for TTN. `xrefs/symbol` takes up to 12 s. The GRCh37 server gives a different canonical transcript | Open |
| MANE summary | `MANE.GRCh38.v1.5.summary.txt.gz` | one 1.1 MB download | Select and Plus Clinical RefSeq and Ensembl pairs per gene | 19,438 rows. HGNC's `mane_select` carries the Select pair for free | Public domain |
| UniProt REST | `uniprotkb/search?query=gene_exact:X AND organism_id:9606 AND reviewed:true` or by accession | 0.5 s | accession, protein name, function, length, subcellular location | `gene_exact` matches synonyms. One gene can have several reviewed entries (CDKN2A has two). Full TTN entry is 496 KB. Fetch by the accessions HGNC lists, never by text | CC BY 4.0 |
| CIViC GraphQL | `gene(entrezSymbol:"X")` or `genes(entrezSymbols:[...])` | 0.2 to 0.3 s | CIViC gene ID, description, variant count, link, sources | Exact official symbol only. HER2 and braf return null. VSIR exists with no variants and no description; ABCC8 is absent, so "absent" and "present but empty" differ. One evidence query returned HTTP 500 | CC0 |
| ClinGen | gene-validity and gene-dosage CSV downloads | 1.9 to 2.8 s each | validity classification, disease and MONDO ID, mode of inheritance, date, expert panel; haploinsufficiency and triplosensitivity | The API ignores its filter and returns all 3714 rows. The CSV has a 4-line banner | CC0 |
| GenCC | submissions TSV | 2.8 s, 26.5 MB | Submitter-level validity keyed by HGNC ID | No per-gene API | CC0 |
| gnomAD GraphQL | `gene(gene_symbol, reference_genome: GRCh38){gnomad_constraint{...}}` | 1.1 s cold, 0.13 s warm | pLI, LOEUF (oe_lof_upper), mis_z, flags, transcript | Official symbol only. GRCh37 serves v2 constraint with different values (BRCA2 LOEUF 0.635 against 0.824) | CC0 |
| Human Protein Atlas | `/ENSG….json` | 0.5 s | tissue and cancer specificity, subcellular location, protein class | Text search is fuzzy (BRAF also returns HMG20B), so fetch by Ensembl ID | [CC BY 4.0](https://www.proteinatlas.org/about/licence), with separate third-party conditions |
| Reactome | `/ContentService/data/mapping/UniProt/<acc>/pathways?species=9606` | 0.8 s | lowest-level pathway IDs and names | Needs the UniProt accession first | [CC0 database data](https://reactome.org/license), checked 2026-10-24; graphics and software have separate terms |
| OncoKB | Keyed source; see section 7 | not measured | gene type, highest sensitivity and resistance levels, gene summary and background, OncoKB transcript per build | The investigation fetched the public gene list once on 2026-10-23, before Ian's keyed-source ruling. From now on BioMCP and its developers call OncoKB only with a user's key | Restricted; user's license |
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
| Gene role in cancer, highest OncoKB levels, OncoKB gene summary | OncoKB, with the user's key only | The only source with a curated oncogene or tumor suppressor call in the examples. Example 14 needs it |
| Withdrawn and merged symbols with successors | HGNC `withdrawn.txt` | The complete set leaves them out |

### Requests and speed for the design below

- Default card: the HGNC bundle resolves the input with no request, and one MyGene.info gene call of about 0.35 s cold adds the summary and spans. With an OncoKB key, one OncoKB call runs beside it.
- Each section adds one request that runs in parallel: UniProt 0.5 s, CIViC 0.3 s, gnomAD 1.1 s cold, HPA 0.5 s, Reactome 0.8 s after UniProt. ClinGen reads a local bundle.
- `get gene X all` should finish in about 1.5 s cold, set by gnomAD and Reactome, against 2.3 to 3.0 s in 0.9.

## 4. Delight questions

1. **What does the agent ask first, and which command answers it?** "What is this gene?" given whatever the clinician typed: a symbol, an alias, an old symbol or an ID. `get gene <text>` answers it. `search gene <text>` answers "which genes match this word or band".
2. **What is the smallest header that lets the agent choose its next step?** HGNC ID, symbol, name, locus group, cytoband, status, and how the input matched: symbol, previous symbol, alias, or ambiguous. These come from one HGNC record.
3. **Which drill-downs does the agent need next, and does `next` name each one?** The card's `next` names the sections with data behind them, chosen from the identity record: `protein` only when HGNC lists UniProt accessions, `civic` only when CIViC has the gene. Pivots to trials, articles, variants and drugs appear in `next` once those slices land, as `search trial --gene ERBB2` and similar.
4. **Which identifiers must come back exact?** The HGNC ID and approved symbol for every later command. The Entrez ID for PubMed and ClinVar. The Ensembl gene ID for HPA and gnomAD. The UniProt accessions for Reactome and the protein slice. The MANE Select RefSeq and Ensembl transcripts with versions for variant naming.
5. **Where do sources disagree, and how does the output show it?** HGNC aliases and previous symbols are the printed lists. NCBI-only aliases serve resolution and say so in `match.kind` (`ncbi_alias`), and the card does not print them. OncoKB's transcript sits in the OncoKB block beside HGNC's MANE pair, each with its version. Transcripts carry versions. Coordinates carry their build and counting rule. The output never merges or picks between two sources' values.
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

Closed sets: `GeneStatus` (approved, withdrawn), `LocusGroup` (protein coding, non-coding RNA, pseudogene, other, phenotype; non-exhaustive), `MatchKind` (symbol, previous symbol, alias, NCBI alias, HGNC ID, Entrez ID, Ensembl ID, band).

`GeneInput` is free text that the edge parses into one of: an `HgncId`, an `EnsemblGeneId`, an `EntrezGeneId` (digits only), or a symbol-like text. Alias text stays text because aliases hold spaces, slashes and parentheses ("HER-2/neu", "MLN 19").

`Gene` (schema `biodata.gene/1`), built from HGNC with MyGene.info fields added:

- `hgnc_id: HgncId`, `symbol: GeneSymbol`, `name`, `status: GeneStatus`, `locus_group: LocusGroup`, `locus_type`
- `location: Cytoband` (absent for genes with no band)
- `previous_symbols: [GeneSymbol]`, `aliases: [text]`. NCBI-only aliases feed resolution and are not printed; for KRAS they are 17 entries and about 85 tokens
- `entrez_id`, `ensembl_id`, `uniprot: [UniProtAcc]`, `omim: [OmimId]`
- `mane_select: { refseq: RefSeqTranscript, ensembl: EnsemblTranscript }`
- `spans: [GenomicInterval]`, one per build, each with strand
- `summary: { text, source: refseq, dated }` (whole text, never cut)

`GeneHeader` (schema `biomcp.gene_header/1`): `hgnc_id`, `symbol`, `name`, `locus_group`, `location`, `status`, and `match: { kind, text }`.

`GeneResolution`: `resolved { gene, match }`, `ambiguous { candidates: [GeneHeader] }`, `withdrawn { hgnc_id, symbol, hgnc_status, merged_into: [HgncId] }`, or `not_found`. `hgnc_status` is "Entry Withdrawn" or "Merged/Split", from HGNC `withdrawn.txt`.

### Summary and sections

The default card is the identity record, the spans, the MANE Select pair and the full RefSeq summary. It measured 1.8 KB of JSON for BRAF (section 6). With an OncoKB key, the card adds a short `oncokb` block: gene type and the highest sensitivity and resistance levels. Without a key, the envelope names OncoKB as skipped once, as section 7 says.

Sections, a closed enum:

| Section | Source | Holds | Not applicable when |
| --- | --- | --- | --- |
| `protein` | UniProt, by HGNC accessions | per protein: accession, name, length, function text, subcellular location | HGNC lists no UniProt accession |
| `civic` | CIViC | CIViC gene ID, description, variant count, link | never; absent and empty are both answers |
| `clingen` | ClinGen bundle | validity rows (disease, MONDO ID, inheritance, classification, date, panel), haploinsufficiency, triplosensitivity | never |
| `constraint` | gnomAD v4 GRCh38 | pLI, LOEUF, mis_z, transcript, version | locus group is not protein coding |
| `expression` | HPA | RNA tissue specificity, top tissues, subcellular location, release | HPA has no record for the Ensembl ID |
| `pathways` | Reactome | pathway ID and name | no UniProt accession |
| `oncokb` | OncoKB, user's key only | OncoKB gene summary and background text, OncoKB transcript per build, data version | OncoKB does not curate the gene (state confirmed empty); no key (state skipped) |

`all` requests every section. Requested sections report one of the four states, or `skipped` for a keyed source with no key. Unrequested sections are absent.

### Command tree

| Command | Returns | Notes |
| --- | --- | --- |
| `biomcp search gene <text>` | `GeneHeader` page, ranked: exact symbol, previous symbol, alias, then name words; protein coding before other locus groups | Ambiguous aliases come back as several headers, each with its `match` |
| `search gene --band 9p21.3` | Headers for genes in a cytoband | From HGNC `location` in the bundle; no live API filters by band. Answers "what else sits in this deleted band" (CDKN2A, CDKN2B, MTAP) |
| `search gene kinase --locus-group protein_coding` | Filter | Replaces 0.9 `--type` |
| `biomcp get gene <input>...` | One card per input, with its `GeneResolution` | Accepts symbols, previous symbols, aliases, `HGNC:` IDs, Entrez IDs and Ensembl IDs. Several inputs at once (HRR and Lynch panels) |
| `get gene <input> <section>...` | Card plus sections | `protein`, `civic`, `clingen`, `constraint`, `expression`, `pathways`, `oncokb`, `all` |

`next` on a resolved card is one step, `get gene <hgnc_id>`, with the list of sections that have data behind it, so the gene ID is not repeated for each section. On an ambiguous result, it names `get gene <hgnc_id>` for each candidate. On a header page, it names `get gene` for the first header.

### Keep, change or drop each 0.9 feature

| 0.9 feature | Ruling | Reason |
| --- | --- | --- |
| `search gene <q>` | Keep, change | Rank by match kind from HGNC. Return HGNC ID, band, locus group and match kind. Examples 7 and 8 need ranked candidates |
| `--type` | Rename `--locus-group` | HGNC's closed set replaces MyGene.info's free text |
| `--chromosome` | Drop | `--band` covers it with a real case. 0.9's client-side filter broke totals |
| `--region` | Drop | Coordinate search belongs to the variant slice's copy-number questions. 0.9's help example returns nothing |
| `--pathway`, `--go` | Drop | No harvested example. Pathway membership belongs to the pathway slice |
| `--limit`, `--offset` | Keep | Bounded pages with an offset and a generated next step |
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
| `gencc` | Ship | Stored GenCC submissions retain submitter, disease, inheritance, classification and dates. `sync gencc` installs or refreshes the complete CSV bundle. |
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

Kept: HGNC bundle (complete set and withdrawn file), MyGene.info, UniProt, CIViC, ClinGen bundle, gnomAD, HPA, Reactome. NCBI E-utilities stays as the check for replaced Entrez IDs.

Dropped from the gene slice: Open Targets, Enrichr, QuickGO, STRING, GTEx, DisGeNET, NIH RePORTER, GTR, KEGG, CSpec, DGIdb (to the drug slice).

Keyed: OncoKB runs only with the user's key, as section 7 says.

## 6. Progressive disclosure and token budget

### How it was measured

The original 1.0 design estimates below use bytes divided by 4. Before implementation, a scratch script built each proposed 1.0 output in the shape section 5 gives, as compact JSON with nulls and empty lists left out, from live replies on 2026-10-24: the HGNC bundle and withdrawn file, MyGene.info, UniProt, CIViC, the ClinGen files, gnomAD, HPA and Reactome. The 0.9.3 numbers are the real outputs of the same questions, in `--json` and in Markdown. 0.9 `pathways` and `all` were not run, because they call KEGG. OncoKB was not called, because no key is configured.

"Facts" counts the facts a careful answer to the case needs, checked as exact text in each output: for KRAS these are the HGNC ID, band, Entrez ID, Ensembl ID, previous symbol KRAS2, MANE RefSeq, UniProt accession, GRCh38 start, GRCh37 start and the RefSeq summary date.

### Layers, densest and highest priority first

| Layer | What it holds | Tokens | Why it sits here |
| --- | --- | --- | --- |
| 1. `search gene` header | HGNC ID, symbol, name, locus group, band, match kind | about 45 per header | Answers "which gene is this" and settles ambiguity before any card is fetched. Every field is an exact identifier or a closed value |
| 2. `get gene` default card | Identity, previous symbols and aliases, cross IDs, MANE Select pair, spans on both builds, full dated RefSeq summary, one `next` step listing sections | 300 to 520 per gene | Answers the common question alone: what the gene is, where it sits, which transcript names its variants, and what RefSeq says it does in cancer. The summary is 46% of the BRAF card and is the only prose; Ian ruled it stays whole |
| 2a. OncoKB block in the card, with a key | Gene type, highest sensitivity level, highest resistance level | about 30, estimated | The densest clinical fact the gene card can carry: oncogene or tumor suppressor, and whether any level 1 therapy exists for the gene. Without a key, one `skipped` note of about 30 tokens per response |
| 3. `constraint` | pLI, LOEUF, missense Z, transcript, version | about 50 | Smallest section; answers germline questions such as example 13 |
| 4. `expression` | HPA tissue and cancer specificity, top tissues, location | about 80 | Small and answers target-toxicity questions such as example 17 |
| 5. `protein` | Each reviewed protein: name, length, function text, location | 155 for BRAF, 1,130 for CDKN2A | Function text is long and two-protein genes double it, so it stays behind a section |
| 6. `clingen` | Validity rows and dosage | 230 for BRAF, 120 to 170 per Lynch gene | Germline questions only; rows grow with curated diseases |
| 7. `pathways` | Reactome pathway IDs and names | 305 for BRAF (16 pathways) | Long lists that serve grouping questions, not the common question |
| 8. `civic` | CIViC gene ID, description, variant count, link | 470 for BRAF, 735 for ERBB2 | Densest cancer evidence prose but long; variant-level evidence belongs to the variant slice |
| 9. `oncokb` section, with a key | OncoKB gene summary and background, OncoKB transcript | about 400, estimated from the 1.9 MB curated-gene list across 1,144 genes | Long licensed prose, so it stays behind a named section even when the key is present |
| Options | `--band`, `--locus-group`, several inputs, page offset | per header or per card | Each option narrows or batches; none adds fields |

### Token table, 1.0 against 0.9.3

| Case | Facts needed | 0.9 JSON tokens | 0.9 JSON facts | 0.9 Markdown tokens | 0.9 Markdown facts | 1.0 tokens | 1.0 facts | 1.0 shipped |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `get gene KRAS` | 10 | 1056 | 4 | 406 | 4 | 392 | 10 | 438 |
| `get gene HER2` | 6 | 1060 | 2 | 406 | 2 | 521 | 6 | 567 |
| `get gene MLL` | 5 | 1055 | 1 | 405 | 1 | 503 | 5 | 577 |
| `get gene FAM46C` | 4 | 990 | 1 | 354 | 1 | 303 | 4 | 354 |
| `get gene BRAF` | 7 | 1062 | 1 | 411 | 1 | 457 | 7 | 508 |
| `get gene HGNC:1097` | 3 | 54 (error) | 1 | 28 (error) | 1 | 460 | 3 | 512 |
| `get gene MLL2` (ambiguous) | 5 | 238 | 4 | 61 | 4 | 154 | 5 | 187 |
| `get gene p16` (ambiguous) | 4 | 241 | 2 | 66 | 2 | 150 | 4 | 184 |
| `get gene BRCA3` (withdrawn) | 2 | 194 | 0 | 57 | 0 | 58 | 2 | 77 |
| `get gene NOTAGENE` | 1 | 44 | 1 | 20 | 0 | 41 | 1 | 55 |
| `search gene p16` | 3 | 602 | 2 | 240 | 2 | 130 | 3 | 163 |
| `search gene KRAS` | 3 | 163 | 1 | 105 | 1 | 88 | 3 | 116 |
| `get gene CDKN2A protein` | 2 | 1332 | 1 | 364 | 1 | 1133 | 2 | 1,187 |
| `get gene MIR21 protein constraint` | 3, and no wrong protein | 1529 (orangutan PARN) | 0 | 591 | 0 | 516 | 3 | not measured in this section order; `constraint protein`: 564 |
| `get gene VISTA civic` | 3 | 1096 | 1 | 229 | 1 | 407 | 3 | 451 |
| `get gene ERBB2 civic` | 4 | 2689 | 0 | 280 | 0 | 735 | 4 | 780 |
| `get gene TP53 constraint clingen` | 6 | 1401 | 5 | 404 | 5 | 585 | 6 | 719 |
| `get gene CLDN18 expression` (0.9 `hpa`) | 5 | 2158 | 4 | 679 | 4 | 572 | 5 | 620 |
| Lynch, five genes with `clingen` (0.9: five calls) | 7 | 6651 | 2 | 1965 | 2 | 2901 | 7 | 3,659 |
| `search gene --band 9p21.3`, first 10 of 65 | 3 | no 0.9 command | | | | 464 | 2 on page 1, 3 in the walk | 540 |

Shipped values above come from the recorded cases on 2026-10-24, using the runner's compact UTF-8 JSON size rounded up after division by four. They measure the whole response, including its envelope and `next`. The design estimates remain historical. `protein-mir21.yaml` requests the same two sections in the order `constraint protein`; its size is measured separately from the unrecorded table spelling.

Section cost on BRAF, as tokens added to the proposed default card:

| Section | 1.0 | 0.9 JSON added | 0.9 Markdown section | 1.0 shipped |
| --- | --- | --- | --- | --- |
| `constraint` | 48 | 110 | 249 | BRAF not recorded; TP53 section: 56 |
| `expression` (0.9 `hpa`) | 83 | 1081 | 590 | BRAF not recorded; CLDN18 section: 108 |
| `protein` | 155 | 219 | 370 | BRAF not recorded; CDKN2A two-protein section: 547 |
| `clingen` | 229 | 323 | 401 | BRAF not recorded; TP53 section: 148; Lynch sections: 243–337 |
| `pathways` | 305 | not run (KEGG) | not run | BRAF section: 295 |
| `civic` | 468 | 1975 | 310 | BRAF not recorded; ERBB2 section: 198 |
| `oncokb` | about 400, estimated | none | none | Real key not measured; synthetic TP53 section: 65 (shape only) |

Original BRAF design estimate by part: summary 845 bytes, identifiers and names 520, spans on two builds 213, the `next` step 118, envelope 79, MANE 54. Two trims are already in: NCBI-only aliases are used for matching and not printed (saves 85 tokens on KRAS), and `next` is one step with a section list instead of six commands (saves about 45 tokens per card).

### Where the original 1.0 design cost more, and why

- **Against 0.9 Markdown on HER2, MLL and BRAF cards** (521, 503 and 457 against about 406). 1.0 carries every needed fact (6, 5 and 7) where 0.9 carries 2, 1 and 1. The extra cost is the full summary Ian ruled in, both builds, the MANE pair and the match kind. JSON is the default output, and against 0.9 JSON every 1.0 card is 50 to 70% smaller.
- **`get gene HGNC:1097`** (460 against 54). 0.9 refuses the ID with an error; 1.0 returns the card.
- **Ambiguous and withdrawn inputs against 0.9 Markdown** (150 against 66 for p16). 0.9 returns UMLS concept labels; 1.0 returns each HGNC candidate with ID, band, locus group and match kind, which lets the agent pick without another search.
- **`civic` against 0.9 Markdown** (468 against 310 on BRAF, 735 against 280 on ERBB2). 0.9 Markdown prints an assertion table without the gene description; 1.0 prints CIViC's description, which names the gene's cancer role. Against 0.9 JSON, 1.0 `civic` is 4 times smaller.
- **Lynch panel against 0.9 Markdown** (2901 against 1965). 1.0 returns all five validity tables with MONDO IDs and dates; 0.9 Markdown carries 2 of 7 facts.

### Budget

The cases in section 8 enforce a byte ceiling on each output, set at about 1.5 times the measured size so that upstream summary edits do not fail them and real growth does. The shipped cases enforce section budgets on each section object, including its state and source; the search and ambiguity budgets apply to individual headers, while the default-card budget applies to each single-card response. Per-command budgets: a search header 60 tokens, a default card 650 tokens, `constraint` 100, `expression` 150, `protein` 400 per protein, `clingen` 600, `pathways` 600, `civic` 1,000, `oncokb` 700, an ambiguous result 60 tokens per candidate, a withdrawn or not-found result 100.

The section ceilings appear in `constraint.yaml`, `expression-cldn18.yaml`, `protein-data.yaml` (two proteins), `clingen-validity.yaml` (each gene), `pathways-data.yaml`, `civic-data.yaml` and `oncokb-data.yaml` under `tests/cases/gene/sections/`. The OncoKB case uses invented text and proves the wire shape and ceiling only. A key holder must still verify the real-key case outside the repo; no licensed-content token result is claimed. Shipped measurements for a BRAF section without a recorded BRAF case remain unmeasured.

## 7. Keyed sources

`decisions/2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` governs every source below. Without the user's key, BioMCP sends no request to the source, including its public endpoints, and the envelope names the source once as skipped, with the config key to set.

### OncoKB

OncoKB is the gene slice's main keyed source. It answers example 14 (oncogene or tumor suppressor) and adds whether a gene has any therapeutic level, which no open source in this slice offers. The fields below come from OncoKB's public API source code (`oncokb/oncokb` on GitHub: `GenesApi`, `UtilsApi`, `apiModels/CuratedGene`, `model/Gene`, `model/GeneType`), not from calls.

| Endpoint | Returns | Use |
| --- | --- | --- |
| `GET /api/v1/utils/allCuratedGenes?hugoSymbol=X&includeEvidence=true` | A list of `CuratedGene`: hugoSymbol, entrezGeneId, geneType, highestSensitiveLevel, highestResistanceLevel, summary, background, grch37Isoform, grch37RefSeq, grch38Isoform, grch38RefSeq | The card's OncoKB block and the `oncokb` section, in one call per gene. The parameter accepts aliases, but BioMCP sends the HGNC symbol |
| `GET /api/v1/genes/{entrezGeneId}` | `Gene`: hugoSymbol, entrezGeneId, geneType, isoforms and RefSeqs per build, geneAliases | Fallback when the curated-gene call is not allowed for the key's plan |
| `GET /api/v1/genes/{entrezGeneId}/evidences?evidenceTypes=GENE_SUMMARY,GENE_BACKGROUND` | Gene-level evidence text | Not needed: `allCuratedGenes` carries the same summary and background |
| `GET /api/v1/info` | Data version and OncoTree version | The `oncokb` block's version, cached per day |
| `annotate/*`, `allActionableVariants` | Variant-level levels and therapies | The variant slice, not this one |

Closed set for gene type, from `GeneType`: `oncogene_and_tsg`, `oncogene`, `tsg`, `insufficient_evidence`, `neither`. The card shows it as OncoKB states it and never derives a role from other sources.

The card block, with a key: `oncokb: { gene_type, highest_sensitive_level, highest_resistance_level, version }`, about 30 tokens. The `oncokb` section adds the summary, the background and OncoKB's transcript per build, about 400 tokens estimated. OncoKB's transcript sits beside HGNC's MANE pair and is never merged with it: BRAF showed NM_004333.4 in OncoKB's list and NM_004333.6 as MANE Select.

States: `data`; `empty` when OncoKB does not curate the gene (VSIR and CLDN18 were absent from its gene list); `unavailable` with the reason on a source error or a rejected key; `skipped` with `reason: no_key` and the config key `oncokb_api_key` when no key is set.

Cases: keyed-source recordings hold licensed content, so they never enter the repo. Recorded OncoKB cases run from outside the repo through the runner's `--cases DIR`. The repo holds only the no-key case, which proves no OncoKB request is sent.

### Other keyed sources

| Source | Key | Serves | Ruling sought |
| --- | --- | --- | --- |
| COSMIC Cancer Gene Census | COSMIC account token | Role in cancer, tier, hallmark, and translocation partner genes, which fill the fusion-partner gap in examples 4 and 15 | Dropped by Ian: BioMCP uses free gene sources, and its only COSMIC data is the old pre-license copy in MyVariant.info |
| NCBI E-utilities | `ncbi_api_key`, optional | Raises the limit from 3 to 10 requests per second for the replaced-ID check | Not a keyed-only source: it runs without a key at the lower limit |
| DisGeNET | API key | Scored gene-disease pairs | Stays dropped by Ian's ruling; ClinGen covers validated links |
| OMIM | API key | Gene-phenotype map | Not needed: no example asks for it beyond what ClinGen and the RefSeq summary give |

## 8. Outside-in cases

Each case was verified on 2026-10-23 or 2026-10-24 against live HGNC, MyGene.info, CIViC or the ClinGen files. Each asserts stable identity facts, ordering rules, states and a byte ceiling, not free text that drifts. Ceilings are about 1.5 times the measured size in section 6.

| # | Story | Run | Expect | Ceiling |
| --- | --- | --- | --- | --- |
| 1 | KRAS G12C lung tumor, gene card first (PMID 34096690) | `get gene KRAS` | `/status` complete; `hgnc_id` HGNC:6407; `entrez_id` 3845; `ensembl_id` ENSG00000133703; `location` 12p12.1; `previous_symbols` includes KRAS2; `mane_select.refseq` starts NM_004985.; `match.kind` symbol | 2,400 bytes |
| 2 | Oncologist types HER2 (PMID 29846122) | `get gene HER2` | `symbol` ERBB2; `hgnc_id` HGNC:3430; `location` 17q12; `match.kind` alias; `match.text` HER2 | 3,200 bytes |
| 3 | MLL-rearranged leukemia (PMID 18022353) | `get gene MLL` | `symbol` KMT2A; `hgnc_id` HGNC:7132; `match.kind` previous_symbol | 3,000 bytes |
| 4 | Myeloma panel says FAM46C (PMID 28931820) | `get gene FAM46C` | `symbol` TENT5C; `hgnc_id` HGNC:24712; `location` 1p12; `match.kind` previous_symbol | 1,900 bytes |
| 5 | Old paper says MLL2 (PMID 18022353) | `get gene MLL2` | resolution ambiguous; candidates exactly {HGNC:7133, HGNC:15840}; KMT2D (previous symbol) ranks before KMT2B (alias); `next` holds `get gene HGNC:7133` and `get gene HGNC:15840` | 1,000 bytes |
| 6 | Pathology says p16 loss (PMID 32747822) | `get gene p16` | resolution ambiguous; candidates include HGNC:1787 and HGNC:43802; CDKN2A (protein coding) ranks first | 1,000 bytes |
| 7 | Family asks about BRCA3 (PMID 16614877) | `get gene BRCA3` | resolution withdrawn; `hgnc_id` HGNC:18617; `hgnc_status` Entry Withdrawn; no card; never BRCA1 or BRCA2 | 400 bytes |
| 8 | Nonsense input | `get gene NOTAGENE` | resolution not_found; error code `not_found`; the input text never appears in the message | 400 bytes |
| 9 | Agent holds an ID from another tool | `get gene HGNC:1097`, `get gene 673`, `get gene ENSG00000157764` | each returns `symbol` BRAF with `match.kind` hgnc_id, entrez_id, ensembl_id | 2,800 bytes each |
| 10 | BRAF summary keeps the cancer sentence (0.9 finding 1) | `get gene BRAF` | `summary.text` includes "V600E"; `summary.dated` present | 2,800 bytes |
| 11 | CDKN2A encodes two proteins (0.9 finding 3) | `get gene CDKN2A protein` | `protein` state data; accessions exactly {P42771, Q8N726} | 6,800 bytes |
| 12 | MIR21 has no protein (0.9 finding 2) | `get gene MIR21 protein constraint` | `locus_group` non_coding_rna; `protein` state not_applicable; `constraint` state not_applicable; no accession anywhere | 3,100 bytes |
| 13 | VISTA in trial text (PMID 24691993) | `get gene VISTA civic` | `symbol` VSIR; `civic` state confirmed empty with CIViC gene ID 14308 | 2,500 bytes |
| 14 | Lynch panel (PMID 25070057) | `get gene MLH1 MSH2 MSH6 PMS2 EPCAM clingen` | five cards in input order; each `clingen` state data; each has a Definitive row for a Lynch syndrome disease | 17,500 bytes |
| 15 | 9p21 deletion, which genes go with CDKN2A | `search gene --band 9p21.3`, walked to the end | the walk includes CDKN2A, CDKN2B and MTAP; total between 55 and 75 (65 on 2026-10-24); no duplicates | 2,800 bytes per page of 10 |
| 16 | CIViC fails during a card | `get gene ERBB2 civic` with a recorded HTTP 500 | `/status` partial; `civic` state unavailable with reason; identity fields still present | 3,200 bytes |
| 17 | TP53 germline finding (PMID 32457520) | `get gene TP53 constraint clingen` | `constraint` state data with version v4; `clingen` holds a Definitive Li-Fraumeni syndrome row with MONDO:0018875 | 3,600 bytes |
| 18 | CLDN18.2-positive gastric cancer (PMID 37068504) | `get gene CLDN18 expression` | `symbol` CLDN18; `expression` state data; stomach is among the tissue-specific entries | 3,500 bytes |
| 19 | NOTCH1 role asked with no OncoKB key (PMID 28890946) | `get gene NOTCH1 oncokb` with no key configured | `oncokb` state skipped, reason no_key, names the config key; the replay records no request to OncoKB; `/status` complete | 3,000 bytes |
| 20 | NOTCH1 role with a key, outside the repo | same, with a key, through `--cases DIR` | `oncokb.gene_type` present as OncoKB states it; `oncokb` section version present | 5,000 bytes |

Cases 5, 6 and 15 assert sets and order rules that BioMCP owns, so upstream additions do not break them. HGNC REST fetch gives CDKN2A, CDKN2B and MTAP each the location 9p21.3. Neither HGNC REST search nor MyGene.info can query by band (MyGene.info answers "map_location is not indexed"), so case 15 and `--band` need the HGNC bundle. Case 7 needs HGNC's `withdrawn.txt`, because the complete set leaves BRCA3 out. Case 20's answer must be verified by a key holder when it is recorded.

## 9. Ian's rulings

Ian approved every recommendation below on 2026-10-24, as `decisions/2026-10-24-gene-slice-rulings.md` records. Question 3 was later widened: OncoKB is allowed as a keyed source that runs only with the user's key, as `decisions/2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` says.

1. **Should HGNC ship as a local bundle?** Recommendation: yes. The CC0 file is 17 MB, changes about weekly, and has an ETag. It makes resolution and `search gene` instant and offline, gives deterministic ambiguity and withdrawn answers, saves one request on every gene, variant and article call that names a gene, and is the only way to offer `search gene --band`, since no live API filters by band. The cost is a `biomcp sync hgnc` step and 17 MB of disk. Without it, every gene input costs one 0.2 to 0.5 s HGNC call. **Answered: yes. HGNC ships as a local bundle.**
2. **When an alias matches several genes, should BioMCP pick one?** Recommendation: never pick. Return ranked candidates with how each matched. MLL2 means KMT2D in older papers and is an alias of KMT2B in HGNC, and p16 matches CDKN2A and a pseudogene. A wrong silent pick would send the agent to the wrong gene, which is what issue 113 reported. **Answered: never pick; return ranked candidates.**
3. **May BioMCP show OncoKB's public gene list fields, such as oncogene or tumor suppressor?** Recommendation: leave OncoKB out until you rule on its license. The endpoint needs no token, but OncoKB terms limit free use to academic research. No open source carries a gene's role in cancer, so example 14 stays answered by CIViC's description and articles until you decide. **Answered: at first leave out; then allowed as a keyed source run only with the user's key (section 7).**
4. **Should the gene-scoped pivots (`gene trials`, `gene articles`, `gene drugs`) stay as gene commands?** Recommendation: move each to its own entity's search (`search trial --gene ERBB2`), and have the gene card's `next` name them. `architecture.md` uses `gene trials BRAF` as an example, so this needs your word. One spelling per idea favors the entity search, and 0.9's `gene drugs HER2` returning nothing shows the cost of two paths. **Answered: move to each entity's search. Retired 0.9 spellings return a usage error naming the new command.**
5. **Do you accept the drops from 0.9?** Enrichr ontology and diseases, GO, STRING, GTEx, DisGeNET, NIH funding, the Open Targets drug and disease lists, `--chromosome`, `--region`, `--pathway`, `--go` and the command aliases. Recommendation: drop them. No harvested example needs them, several return noise or wrong answers, and the parity bar covers useful behavior only. **Answered: yes, the drops stay dropped.**
6. **Should the default card carry the full RefSeq summary?** Recommendation: yes, whole and dated. It is about 800 characters for BRAF and holds the sentence that names V600E and the cancers. 0.9's two-sentence cut removed it. **Answered: yes, whole and dated.**
7. **Should the card give both GRCh38 and GRCh37 spans?** Recommendation: yes. MyGene.info returns both in the same call at no cost, many lab reports and OncoKB's list still carry GRCh37 fields, and each span states its build. **Answered: yes, both builds.**
8. **Should BioMCP serve non-human genes?** Recommendation: human only. Every example is human. HGNC maps mouse Braf to human BRAF, and the card should say the input matched a human symbol without regard to case. **Answered: human only.**

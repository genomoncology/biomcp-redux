# Roadmap

BioMCP 1.0 rebuilds search and get on typed biodata entities. This page lists every entity, every source and the foundations as checklists. A box is ticked only in the commit that lands the work, so the roadmap never claims more than main holds. Tickets and their status live in pm. The architecture is in `sdlc/planning/architecture.md`, and the slice method is in `sdlc/planning/slices.md`.

Source tags: **bundle** is a local data file queried like an API. **keyed** runs only with the user's own key. **optional key** works without a key and runs faster or with higher limits with one. **dropped** is not carried into 1.0, with the reason in the slice design. **investigate** waits for the entity's slice to decide.

## Parity with 0.9

| Measure | Count |
| --- | --- |
| 0.9 commands and options | 112 commands, 375 options |
| Ported or renamed | 0 |
| Dropped by ruling | 0 |
| Not yet decided | 487 |

Each slice updates these counts when its accounting lands.

## Foundations

- [ ] Workspace and five crates with enforced dependency direction
- [x] Rust standards, lints and ratchets in the gate
- [x] Minimal biodata core types and conventions
- [ ] Transport: deadlines, source-key rate limits, retries, keyed sources, reply checks, cache
- [ ] Record, replay and offline modes
- [ ] SQLite store: reply cache, pools, bundle rows
- [ ] Bundle sync with lock and budget
- [ ] Schema generation, checked-in schemas and the compatibility check
- [ ] JSON envelope, statuses, exit codes and next steps
- [ ] Markdown view over JSON
- [ ] Outside-in runner with size ceilings and coverage floor
- [ ] The one MCP tool over the CLI grammar
- [ ] Release pipeline carried over from 0.9

## Every entity's checklist

Each entity below repeats these steps.

1. Investigation: real examples, source checks, 0.9 probe
2. Questions answered by Ian
3. Biodata types and schema
4. Search
5. Get and sections
6. Outside-in cases with stories and verified answers
7. Token budget met against 0.9
8. MCP parity
9. Docs
10. 0.9 accounting closed

## Milestone 1: genes and articles

### Gene

- [x] Investigation
- [x] Questions answered by Ian
- [ ] Biodata types and schema
- [ ] Search
- [ ] Get and sections
- [ ] Outside-in cases
- [ ] Token budget
- [ ] MCP parity
- [ ] Docs
- [ ] 0.9 accounting

Sources:
- [ ] HGNC (bundle, identity and names)
- [ ] MyGene.info
- [ ] UniProt
- [ ] Reactome
- [ ] CIViC
- [ ] ClinGen
- [ ] GenCC (bundle)
- [ ] gnomAD (constraint)
- [ ] GTEx
- [ ] Human Protein Atlas
- [ ] OncoKB (keyed)
- [ ] COSMIC Cancer Gene Census (keyed)
- Moved to the drug slice: DGIdb and Open Targets druggability
- Moved to the cell-line slice: Cellosaurus
- Dropped: QuickGO, STRING, Enrichr, DisGeNET, NIH RePORTER, KEGG

### Article

- [ ] Investigation
- [ ] Questions answered by Ian
- [ ] Biodata types and schema
- [ ] Search, with plain PubMed as the floor
- [ ] Get, full text, supplementary material and citations
- [ ] Outside-in cases, including AKT1 E17K
- [ ] Token budget
- [ ] MCP parity
- [ ] Docs
- [ ] 0.9 accounting

Sources:
- [ ] PubMed (optional key)
- [ ] PubTator3 (optional key)
- [ ] Europe PMC
- [ ] Semantic Scholar (optional key)
- [ ] PMC Open Access and NCBI ID converter (optional key)
- [ ] NCBI EFetch (optional key)
- [ ] LitVar (investigate)
- [ ] LitSense2 (investigate)
- [ ] OpenCitations (investigate)
- [ ] Figshare (investigate)

## Milestone 2: variants and trials

### Variant

- [ ] Investigation
- [ ] Questions answered by Ian
- [ ] Biodata types and schema
- [ ] Search
- [ ] Get and sections
- [ ] Outside-in cases
- [ ] Token budget
- [ ] MCP parity
- [ ] Docs
- [ ] 0.9 accounting

Sources:
- [ ] MyVariant.info
- [ ] ClinGen Allele Registry
- [ ] ClinVar through NCBI EFetch (optional key)
- [ ] dbSNP
- [ ] gnomAD
- [ ] CIViC
- [ ] cBioPortal
- [ ] Cancer Hotspots
- [ ] OncoKB (keyed)
- [ ] GWAS Catalog
- [ ] ClinGen evidence repository
- [ ] UniProt and InterPro (investigate)
- [ ] Mutalyzer and VariantValidator (investigate)
- [ ] AlphaGenome (keyed, investigate)

### Trial

- [ ] Investigation
- [ ] Questions answered by Ian
- [ ] Biodata types and schema
- [ ] Search
- [ ] Get and sections
- [ ] Outside-in cases
- [ ] Token budget
- [ ] MCP parity
- [ ] Docs
- [ ] 0.9 accounting

Sources:
- [ ] ClinicalTrials.gov
- [ ] NCI Clinical Trials Search (keyed)
- [ ] MyDisease.info for disease resolution

## Milestone 3: drugs and diseases

### Drug

- [ ] Investigation
- [ ] Questions answered by Ian
- [ ] Biodata types and schema
- [ ] Search
- [ ] Get and sections
- [ ] Outside-in cases
- [ ] Token budget
- [ ] MCP parity
- [ ] Docs
- [ ] 0.9 accounting

Sources:
- [ ] MyChem.info
- [ ] openFDA labels and approvals (optional key)
- [ ] EMA (bundle)
- [ ] WHO prequalified medicines (bundle)
- [ ] FDA orphan designations
- [ ] ChEMBL
- [ ] Open Targets
- [ ] DGIdb
- [ ] CIViC
- [ ] DDInter (bundle, investigate)
- [ ] PharmacoDB (investigate)
- [ ] CVX vaccine codes (bundle, investigate)

### Disease

- [ ] Investigation
- [ ] Questions answered by Ian
- [ ] Biodata types and schema
- [ ] Search
- [ ] Get and sections
- [ ] Outside-in cases
- [ ] Token budget
- [ ] MCP parity
- [ ] Docs
- [ ] 0.9 accounting

Sources:
- [ ] MyDisease.info
- [ ] Monarch
- [ ] HPO
- [ ] OLS
- [ ] SEER
- [ ] Open Targets associations
- [ ] CIViC
- [ ] Reactome (investigate)
- [ ] UMLS (keyed, investigate)
- [ ] MedlinePlus (investigate)
- Dropped with the gene slice ruling unless the disease slice needs them: DisGeNET, NIH RePORTER

## Milestone 4: the remaining entities

Each entity below carries the same ten-step checklist when its slice starts.

### Protein
- [ ] Slice complete
- [ ] UniProt
- [ ] InterPro
- [ ] Complex Portal (investigate)
- [ ] STRING (investigate)

### Pathway
- [ ] Slice complete
- [ ] Reactome
- [ ] WikiPathways
- [ ] g:Profiler (investigate)
- Dropped: KEGG

### Pharmacogenomics
- [ ] Slice complete
- [ ] CPIC
- [ ] PharmGKB

### Diagnostic
- [ ] Slice complete
- [ ] NCBI GTR (bundle)
- [ ] WHO in vitro diagnostics (bundle)
- [ ] openFDA devices (optional key)

### Adverse event
- [ ] Slice complete
- [ ] openFDA FAERS and device reports (optional key)
- [ ] VAERS
- [ ] CVX (bundle)

### Cell line
- [ ] Slice complete
- [ ] Cellosaurus
- [ ] ChEMBL
- [ ] PharmacoDB

### Author
- [ ] Slice complete
- [ ] Semantic Scholar (optional key)
- [ ] ORCID (optional key)

### Phenotype
- [ ] Slice complete
- [ ] Monarch
- [ ] HPO

### GWAS
- [ ] Slice complete
- [ ] GWAS Catalog

### Study
- [ ] Slice complete (get only; analysis and charts are out of scope)
- [ ] cBioPortal studies (bundle)

## Milestone 5: parity and release

- [ ] Every 0.9 command and option ported, renamed or dropped by ruling
- [ ] Docs rewritten
- [ ] Release rehearsal on every platform
- [ ] The current BioMCP repo's history replaced with this repo's

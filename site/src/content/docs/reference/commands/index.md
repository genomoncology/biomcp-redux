---
title: Commands
description: Every BioMCP 1.0.0 command, grouped by entity, with what it does.
goal: Find the reference page for one BioMCP command.
reader: A BioMCP user or agent looking up a command
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources, has 86 commands. They are grouped below by [entity](/reference/entities/), the kind of record they read, such as gene or trial. Each links to its page with arguments, sections and examples.

## adverse-event

| Command | Description |
| --- | --- |
| [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) | Finds adverse event reports in FAERS, VAERS or MAUDE, five at a time. |
| [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) | Reads one adverse event report by its ID, and optionally one section such as its drugs or reactions. |

## article

| Command | Description |
| --- | --- |
| [`biomcp search article`](/reference/commands/search-article/) | Finds articles by text, gene, disease, drug, variant, author, journal or date. |
| [`biomcp get article`](/reference/commands/get-article/) | Reads one to twenty articles by PMID, PMCID or DOI, and optionally sections such as the full text. |
| [`biomcp article supplement`](/reference/commands/article-supplement/) | Reads one supplementary file of an article, or the rows of it that match a search, or saves the file. |
| [`biomcp article pdf`](/reference/commands/article-pdf/) | Downloads an article's PDF from its publisher to a file you name. |
| [`biomcp article text`](/reference/commands/article-text/) | Reads one part of an article's full text, finds matching passages, or saves the text as Markdown. |
| [`biomcp article citations`](/reference/commands/article-citations/) | Lists the papers that cite one PubMed article, a page at a time. |
| [`biomcp article references`](/reference/commands/article-references/) | Lists the papers one PubMed article cites, a page at a time. |
| [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) | Shows where and how one article cites another, with the citing passage when the full text is open. |
| [`biomcp article recommendations`](/reference/commands/article-recommendations/) | Lists papers that Semantic Scholar recommends from one or more seed papers. |

## author

| Command | Description |
| --- | --- |
| [`biomcp author papers`](/reference/commands/author-papers/) | Lists the papers of one author record from Semantic Scholar or ORCID, a page at a time. |
| [`biomcp search author`](/reference/commands/search-author/) | Finds author records by name in Semantic Scholar. |
| [`biomcp get author`](/reference/commands/get-author/) | Reads author records by their Semantic Scholar or ORCID ID, in the order you give. |

## cell-line

| Command | Description |
| --- | --- |
| [`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/) | Counts the ChEMBL assays recorded for one cell line. |
| [`biomcp search cell-line`](/reference/commands/search-cell-line/) | Finds cell lines by name in Cellosaurus. |
| [`biomcp get cell-line`](/reference/commands/get-cell-line/) | Reads one cell line by its ID, and optionally one named section such as its variants. |

## diagnostic

| Command | Description |
| --- | --- |
| [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) | Finds diagnostic tests in one source, such as GTR, the FDA companion diagnostic list or WHO's prequalified tests. |
| [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) | Reads one diagnostic test record, and optionally one section such as its genes or conditions. |

## disease

| Command | Description |
| --- | --- |
| [`biomcp search disease`](/reference/commands/search-disease/) | Finds disease concepts by text, or lists SEER cancer sites. |
| [`biomcp get disease`](/reference/commands/get-disease/) | Reads one disease concept or SEER cancer site, and optionally a section such as its genes or incidence. |

## drug

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Finds drugs by name in one source. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Reads one drug record by its ID in one source, and optionally a section such as its label or approvals. |

## gene

| Command | Description |
| --- | --- |
| [`biomcp get gene`](/reference/commands/get-gene/) | Reads one to twenty human genes, in the order you give, and optionally sections such as constraint or expression. |
| [`biomcp search gene`](/reference/commands/search-gene/) | Finds human genes by text, by cytoband, or both. |

## gwas

| Command | Description |
| --- | --- |
| [`biomcp get gwas`](/reference/commands/get-gwas/) | Reads one GWAS Catalog association or study, and optionally its loci, associations or ancestries. |
| [`biomcp search gwas`](/reference/commands/search-gwas/) | Finds GWAS Catalog associations by trait, gene, variant, study or PubMed ID, smallest p-value first. |

## pathway

| Command | Description |
| --- | --- |
| [`biomcp search pathway`](/reference/commands/search-pathway/) | Finds human pathways in Reactome, the default, or WikiPathways. |
| [`biomcp get pathway`](/reference/commands/get-pathway/) | Reads one Reactome or WikiPathways pathway, and optionally its summary, genes or events. |
| [`biomcp pathway genes`](/reference/commands/pathway-genes/) | Lists the genes and gene products of one pathway. |
| [`biomcp pathway events`](/reference/commands/pathway-events/) | Lists the events, such as reactions, of one Reactome pathway. |

## pgx

| Command | Description |
| --- | --- |
| [`biomcp search pgx`](/reference/commands/search-pgx/) | Finds CPIC gene-drug pairs for a gene. |
| [`biomcp get pgx`](/reference/commands/get-pgx/) | Reads pharmacogenomic records from CPIC or ClinPGx, such as guidelines, recommendations or annotations. |

## phenotype

| Command | Description |
| --- | --- |
| [`biomcp search phenotype`](/reference/commands/search-phenotype/) | Finds Human Phenotype Ontology (HP) terms by text in HPO or Monarch. |
| [`biomcp get phenotype`](/reference/commands/get-phenotype/) | Reads one HP term from HPO or Monarch, and optionally its synonyms, links, related diseases or genes. |

## protein

| Command | Description |
| --- | --- |
| [`biomcp protein features`](/reference/commands/protein-features/) | Lists UniProt features of one protein, five at a time, filtered by type or residue. |
| [`biomcp protein isoforms`](/reference/commands/protein-isoforms/) | Lists the isoforms of one protein, five at a time. |
| [`biomcp protein domains`](/reference/commands/protein-domains/) | Lists the InterPro domains of one protein, five at a time. |
| [`biomcp protein complexes`](/reference/commands/protein-complexes/) | Lists the Complex Portal complexes that hold one protein, five at a time. |
| [`biomcp protein complex-members`](/reference/commands/protein-complex-members/) | Lists the members of one Complex Portal complex that holds the protein, five at a time. |
| [`biomcp protein structures`](/reference/commands/protein-structures/) | Lists the PDB structures of one protein, five at a time. |
| [`biomcp protein sequence`](/reference/commands/protein-sequence/) | Prints the whole amino-acid sequence of one UniProt protein or isoform. |
| [`biomcp search protein`](/reference/commands/search-protein/) | Finds reviewed human UniProt proteins by text or by gene, five at a time. |
| [`biomcp get protein`](/reference/commands/get-protein/) | Reads UniProt protein records by accession, and optionally sections such as domains or sequence. |

## study

| Command | Description |
| --- | --- |
| [`biomcp search study`](/reference/commands/search-study/) | Finds cBioPortal studies by text in a downloaded copy of the study list. |
| [`biomcp get study`](/reference/commands/get-study/) | Reads one cBioPortal study, and optionally its profiles, sample lists or files. |

## trial

| Command | Description |
| --- | --- |
| [`biomcp search trial`](/reference/commands/search-trial/) | Finds clinical trials in ClinicalTrials.gov, the default, or NCI, by condition, drug, gene, location and more. |
| [`biomcp get trial`](/reference/commands/get-trial/) | Reads clinical trials by NCT ID, and optionally sections such as eligibility or locations. |
| [`biomcp trial document`](/reference/commands/trial-document/) | Downloads one document a trial posted on ClinicalTrials.gov, such as its protocol PDF, to a file you name. |

## variant

| Command | Description |
| --- | --- |
| [`biomcp variant assertions`](/reference/commands/variant-assertions/) | Lists the assertions one source, such as ClinVar, makes about a variant, a page at a time. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Reads a variant by rsID, ClinVar ID, ClinGen allele ID or HGVS, and optionally sections such as ClinVar or population. |
| [`biomcp variant population`](/reference/commands/variant-population/) | Lists gnomAD population frequency details for one variant. |
| [`biomcp search variant`](/reference/commands/search-variant/) | Finds variants by gene, protein change, region or another single selector. |
| [`biomcp variant recurrence`](/reference/commands/variant-recurrence/) | Counts the samples that carry one variant in one cBioPortal study. |
| [`biomcp variant normalize`](/reference/commands/variant-normalize/) | Sends an HGVS variant name to the ClinGen Allele Registry, Mutalyzer or VariantValidator to check and normalize it. |

## Setup, keys, data and server

| Command | Description |
| --- | --- |
| [`biomcp skill`](/reference/commands/skill/) | Shows the agent guide and worked examples built into BioMCP. |
| [`biomcp skill list`](/reference/commands/skill-list/) | Lists the worked examples built into BioMCP. |
| [`biomcp skill render`](/reference/commands/skill-render/) | Prints the whole agent guide. |
| [`biomcp skill status`](/reference/commands/skill-status/) | Reports whether the guide installed in a folder is missing, changed, out of date or current. |
| [`biomcp skill install`](/reference/commands/skill-install/) | Installs the agent guide into a folder. |
| [`biomcp version`](/reference/commands/version/) | Prints the version; --verbose adds the commit it was built from, the schema IDs and the paths BioMCP uses. |
| [`biomcp mcp-config`](/reference/commands/mcp-config/) | Prints the configuration an MCP client needs to start BioMCP. |
| [`biomcp store clean`](/reference/commands/store-clean/) | Removes expired replies and stored searches from the local cache. |
| [`biomcp health`](/reference/commands/health/) | Lists every source with its terms and checks that each one answers. |
| [`biomcp sync`](/reference/commands/sync/) | Downloads a source's [bundle](/local-data/), a local copy that commands read. |
| [`biomcp schema`](/reference/commands/schema/) | Prints one reply schema as JSON Schema. |
| [`biomcp help`](/reference/commands/help/) | Prints the help for every command, or for one, as JSON. |
| [`biomcp key list`](/reference/commands/key-list/) | Lists the keys BioMCP supports and where each one is set, without showing values. |
| [`biomcp key set`](/reference/commands/key-set/) | Stores a key for one source, read from a hidden prompt or standard input. |
| [`biomcp key remove`](/reference/commands/key-remove/) | Removes a stored key. |
| [`biomcp serve`](/reference/commands/serve/) | Runs BioMCP as an MCP server over standard input and output, or over HTTP. |
| [`biomcp sync wikipathways`](/reference/commands/sync-wikipathways/) | Downloads the WikiPathways bundle that pathway searches read. |
| [`biomcp sync cbioportal-studies`](/reference/commands/sync-cbioportal-studies/) | Downloads the cBioPortal study list bundle that study commands read. |
| [`biomcp sync ema`](/reference/commands/sync-ema/) | Downloads the European Medicines Agency bundle that EMA drug commands read. |
| [`biomcp sync ddinter`](/reference/commands/sync-ddinter/) | Downloads the DDInter drug interaction bundle. |
| [`biomcp sync who-pq`](/reference/commands/sync-who-pq/) | Downloads the WHO prequalified medicines bundle. |
| [`biomcp sync gtr`](/reference/commands/sync-gtr/) | Downloads the Genetic Testing Registry bundle that diagnostic commands read. |
| [`biomcp sync who-ivd`](/reference/commands/sync-who-ivd/) | Downloads the WHO prequalified diagnostics bundle. |
| [`biomcp sync fda-cdx`](/reference/commands/sync-fda-cdx/) | Downloads the FDA companion diagnostics bundle. |
| [`biomcp help key ncbi`](/reference/commands/help-key-ncbi/) | Shows how to set the NCBI key: the key name and its environment variable. |
| [`biomcp help key semantic-scholar`](/reference/commands/help-key-semantic-scholar/) | Shows how to set the Semantic Scholar key: the key name and its environment variable. |
| [`biomcp help key orcid`](/reference/commands/help-key-orcid/) | Shows how to set the ORCID key: the key name and its environment variable. |
| [`biomcp help key oncokb`](/reference/commands/help-key-oncokb/) | Shows how to set the OncoKB key: the key name and its environment variable. |
| [`biomcp help key alphagenome`](/reference/commands/help-key-alphagenome/) | Shows how to set the AlphaGenome key: the key name and its environment variable. |
| [`biomcp help key nci`](/reference/commands/help-key-nci/) | Shows how to set the NCI key: the key name and its environment variable. |
| [`biomcp help key openfda`](/reference/commands/help-key-openfda/) | Shows how to set the openFDA key: the key name and its environment variable. |

## Help topics

`biomcp help` does not list these topics. A reply names one in `next` when a source fails with reason `proxy` or `tls`.

| Topic | Description |
| --- | --- |
| [`biomcp help network proxy`](/reference/commands/help-network-proxy/) | Shows how to make BioMCP reach the sources through a proxy. |
| [`biomcp help network ca_bundle`](/reference/commands/help-network-ca_bundle/) | Shows how to make BioMCP trust a private certificate authority. |

## Next

[Command grammar](/concepts/progressive-disclosure/) explains how search, get, sections and relation commands fit together.

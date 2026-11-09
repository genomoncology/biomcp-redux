---
title: Commands
description: Every BioMCP 1.0.0 command, grouped by entity, with what it does.
goal: Find the reference page for one BioMCP command.
reader: A BioMCP user or agent looking up a command
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources, has 232 commands. They are grouped below by [entity](/reference/entities/), the kind of record they read, such as gene or trial. Each links to its page with arguments, sections and examples.

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
| [`biomcp article`](/reference/commands/article/) | Read article text, references, citations and files. |

## author

| Command | Description |
| --- | --- |
| [`biomcp author papers`](/reference/commands/author-papers/) | Lists the papers of one author record from Semantic Scholar or ORCID, a page at a time. |
| [`biomcp search author`](/reference/commands/search-author/) | Finds author records by name in Semantic Scholar. |
| [`biomcp get author`](/reference/commands/get-author/) | Reads author records by their Semantic Scholar or ORCID ID, in the order you give. |
| [`biomcp author`](/reference/commands/author/) | Read papers attributed to a source author record. |

## biomarker

| Command | Description |
| --- | --- |
| [`biomcp search biomarker`](/reference/commands/search-biomarker/) | Search NCI biomarker concepts by name or synonym. |
| [`biomcp get biomarker`](/reference/commands/get-biomarker/) | Get NCI biomarker concepts by code. |

## cell-line

| Command | Description |
| --- | --- |
| [`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/) | Counts the ChEMBL assays recorded for one cell line. |
| [`biomcp search cell-line`](/reference/commands/search-cell-line/) | Finds cell lines by name in Cellosaurus. |
| [`biomcp get cell-line`](/reference/commands/get-cell-line/) | Reads one cell line by its ID, and optionally one named section such as its variants. |
| [`biomcp cell-line`](/reference/commands/cell-line/) | Read source cell line assay counts. |

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

## organization

| Command | Description |
| --- | --- |
| [`biomcp search organization`](/reference/commands/search-organization/) | Search clinical research organizations. |

## pathway

| Command | Description |
| --- | --- |
| [`biomcp search pathway`](/reference/commands/search-pathway/) | Finds human pathways in Reactome, the default, or WikiPathways. |
| [`biomcp get pathway`](/reference/commands/get-pathway/) | Reads one Reactome or WikiPathways pathway, and optionally its summary, genes or events. |
| [`biomcp pathway genes`](/reference/commands/pathway-genes/) | Lists the genes and gene products of one pathway. |
| [`biomcp pathway events`](/reference/commands/pathway-events/) | Lists the events, such as reactions, of one Reactome pathway. |
| [`biomcp pathway`](/reference/commands/pathway/) | Read pathway genes and source events. |

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
| [`biomcp protein`](/reference/commands/protein/) | Read protein sequences, features and source relationships. |

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
| [`biomcp trial`](/reference/commands/trial/) | Read trial groups and public documents. |

## variant

| Command | Description |
| --- | --- |
| [`biomcp variant assertions`](/reference/commands/variant-assertions/) | Lists the assertions one source, such as ClinVar, makes about a variant, a page at a time. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Reads a variant by rsID, ClinVar ID, ClinGen allele ID or HGVS, and optionally sections such as ClinVar or population. |
| [`biomcp variant population`](/reference/commands/variant-population/) | Lists gnomAD population frequency details for one variant. |
| [`biomcp search variant`](/reference/commands/search-variant/) | Finds variants by gene, protein change, region or another single selector. |
| [`biomcp variant recurrence`](/reference/commands/variant-recurrence/) | Counts the samples that carry one variant in one cBioPortal study. |
| [`biomcp variant normalize`](/reference/commands/variant-normalize/) | Sends an HGVS variant name to the ClinGen Allele Registry, Mutalyzer or VariantValidator to check and normalize it. |
| [`biomcp variant`](/reference/commands/variant/) | Read source variant assertions, populations and recurrence. |

## Setup, keys, data and server

| Command | Description |
| --- | --- |
| [`biomcp skill`](/reference/commands/skill/) | Shows the agent guide and worked examples built into BioMCP. |
| [`biomcp skill list`](/reference/commands/skill-list/) | Lists the worked examples built into BioMCP. |
| [`biomcp skill render`](/reference/commands/skill-render/) | Prints the whole agent guide. |
| [`biomcp skill status`](/reference/commands/skill-status/) | Reports whether the guide installed in a folder is missing, changed, out of date or current. |
| [`biomcp skill install`](/reference/commands/skill-install/) | Installs the agent guide into a folder. |
| [`biomcp source`](/reference/commands/source/) | List source names as JSON. |
| [`biomcp source list`](/reference/commands/source-list/) | List the source names a hosted request may leave out. |
| [`biomcp version`](/reference/commands/version/) | Prints the version; --verbose adds the commit it was built from, the schema IDs and the paths BioMCP uses. |
| [`biomcp mcp-config`](/reference/commands/mcp-config/) | Prints the configuration an MCP client needs to start BioMCP. |
| [`biomcp store clean`](/reference/commands/store-clean/) | Removes expired replies and stored searches from the local cache. |
| [`biomcp health`](/reference/commands/health/) | Lists every source with its terms and checks that each one answers. |
| [`biomcp sync`](/reference/commands/sync/) | Downloads a source's [bundle](/local-data/), a local copy that commands read. |
| [`biomcp help`](/reference/commands/help/) | Prints the help for every command, or for one, as JSON. |
| [`biomcp key list`](/reference/commands/key-list/) | Lists the keys BioMCP supports and where each one is set, without showing values. |
| [`biomcp key set`](/reference/commands/key-set/) | Stores a key for one source, read from a hidden prompt or standard input. |
| [`biomcp key remove`](/reference/commands/key-remove/) | Removes a stored key. |
| [`biomcp serve`](/reference/commands/serve/) | Runs BioMCP as an MCP server over standard input and output, or over HTTP. |
| [`biomcp help exit`](/reference/commands/help-exit/) | Process exit codes: 0: complete or partial reply 1: no matching record 2: invalid command or arguments 3: expired result pool 4: source request failed 5: local configuration, storage or output failed Error codes: usage.schema, usage.arguments, usage.mcp_unavailable, usage.unknown_command, usage.retired, local.failure, pool.expired, usage.pool_mismatch, usage.too_many_pmids, usage.too_broad, usage.unsupported_filter. |
| [`biomcp search`](/reference/commands/search/) | Search source records by entity and checked filters. |
| [`biomcp get`](/reference/commands/get/) | Retrieve one entity and its source sections. |
| [`biomcp store`](/reference/commands/store/) | Manage local cached replies and snapshots. |
| [`biomcp key`](/reference/commands/key/) | Register and manage source credentials without showing values. |
| [`biomcp schema`](/reference/commands/schema/) | Prints one reply schema as JSON Schema. |
| [`biomcp help search`](/reference/commands/help-search/) | Search source records by entity and checked filters. |
| [`biomcp help search biomarker`](/reference/commands/help-search-biomarker/) | Describe help search biomarker. |
| [`biomcp help search author`](/reference/commands/help-search-author/) | Read papers attributed to a source author record. |
| [`biomcp help search organization`](/reference/commands/help-search-organization/) | Describe help search organization. |
| [`biomcp help search cell-line`](/reference/commands/help-search-cell-line/) | Read source cell line assay counts. |
| [`biomcp help search gwas`](/reference/commands/help-search-gwas/) | Describe help search gwas. |
| [`biomcp help search protein`](/reference/commands/help-search-protein/) | Read protein sequences, features and source relationships. |
| [`biomcp help search phenotype`](/reference/commands/help-search-phenotype/) | Describe help search phenotype. |
| [`biomcp help search pathway`](/reference/commands/help-search-pathway/) | Read pathway genes and source events. |
| [`biomcp help search study`](/reference/commands/help-search-study/) | Describe help search study. |
| [`biomcp help search pgx`](/reference/commands/help-search-pgx/) | Describe help search pgx. |
| [`biomcp help search diagnostic`](/reference/commands/help-search-diagnostic/) | Describe help search diagnostic. |
| [`biomcp help search adverse-event`](/reference/commands/help-search-adverse-event/) | Describe help search adverse-event. |
| [`biomcp help search disease`](/reference/commands/help-search-disease/) | Describe help search disease. |
| [`biomcp help search drug`](/reference/commands/help-search-drug/) | Describe help search drug. |
| [`biomcp help search gene`](/reference/commands/help-search-gene/) | Describe help search gene. |
| [`biomcp help search article`](/reference/commands/help-search-article/) | Read article text, references, citations and files. |
| [`biomcp help search trial`](/reference/commands/help-search-trial/) | Read trial groups and public documents. |
| [`biomcp help search variant`](/reference/commands/help-search-variant/) | Read source variant assertions, populations and recurrence. |
| [`biomcp help get`](/reference/commands/help-get/) | Retrieve one entity and its source sections. |
| [`biomcp help get biomarker`](/reference/commands/help-get-biomarker/) | Describe help get biomarker. |
| [`biomcp help get author`](/reference/commands/help-get-author/) | Read papers attributed to a source author record. |
| [`biomcp help get cell-line`](/reference/commands/help-get-cell-line/) | Read source cell line assay counts. |
| [`biomcp help get gwas`](/reference/commands/help-get-gwas/) | Describe help get gwas. |
| [`biomcp help get protein`](/reference/commands/help-get-protein/) | Read protein sequences, features and source relationships. |
| [`biomcp help get phenotype`](/reference/commands/help-get-phenotype/) | Describe help get phenotype. |
| [`biomcp help get pathway`](/reference/commands/help-get-pathway/) | Read pathway genes and source events. |
| [`biomcp help get study`](/reference/commands/help-get-study/) | Describe help get study. |
| [`biomcp help get pgx`](/reference/commands/help-get-pgx/) | Describe help get pgx. |
| [`biomcp help get diagnostic`](/reference/commands/help-get-diagnostic/) | Describe help get diagnostic. |
| [`biomcp help get adverse-event`](/reference/commands/help-get-adverse-event/) | Describe help get adverse-event. |
| [`biomcp help get disease`](/reference/commands/help-get-disease/) | Describe help get disease. |
| [`biomcp help get drug`](/reference/commands/help-get-drug/) | Describe help get drug. |
| [`biomcp help get gene`](/reference/commands/help-get-gene/) | Describe help get gene. |
| [`biomcp help get article`](/reference/commands/help-get-article/) | Read article text, references, citations and files. |
| [`biomcp help get trial`](/reference/commands/help-get-trial/) | Read trial groups and public documents. |
| [`biomcp help get variant`](/reference/commands/help-get-variant/) | Read source variant assertions, populations and recurrence. |
| [`biomcp help get clingen`](/reference/commands/help-get-clingen/) | Describe help get clingen. |
| [`biomcp help author`](/reference/commands/help-author/) | Read papers attributed to a source author record. |
| [`biomcp help author papers`](/reference/commands/help-author-papers/) | Describe help author papers. |
| [`biomcp help cell-line`](/reference/commands/help-cell-line/) | Read source cell line assay counts. |
| [`biomcp help cell-line assay-count`](/reference/commands/help-cell-line-assay-count/) | Describe help cell-line assay-count. |
| [`biomcp help protein`](/reference/commands/help-protein/) | Read protein sequences, features and source relationships. |
| [`biomcp help protein features`](/reference/commands/help-protein-features/) | Describe help protein features. |
| [`biomcp help protein isoforms`](/reference/commands/help-protein-isoforms/) | Describe help protein isoforms. |
| [`biomcp help protein domains`](/reference/commands/help-protein-domains/) | Describe help protein domains. |
| [`biomcp help protein complexes`](/reference/commands/help-protein-complexes/) | Describe help protein complexes. |
| [`biomcp help protein complex-members`](/reference/commands/help-protein-complex-members/) | Describe help protein complex-members. |
| [`biomcp help protein structures`](/reference/commands/help-protein-structures/) | Describe help protein structures. |
| [`biomcp help protein sequence`](/reference/commands/help-protein-sequence/) | Describe help protein sequence. |
| [`biomcp help pathway`](/reference/commands/help-pathway/) | Read pathway genes and source events. |
| [`biomcp help pathway genes`](/reference/commands/help-pathway-genes/) | Describe help pathway genes. |
| [`biomcp help pathway events`](/reference/commands/help-pathway-events/) | Describe help pathway events. |
| [`biomcp help article`](/reference/commands/help-article/) | Read article text, references, citations and files. |
| [`biomcp help article supplement`](/reference/commands/help-article-supplement/) | Describe help article supplement. |
| [`biomcp help article pdf`](/reference/commands/help-article-pdf/) | Describe help article pdf. |
| [`biomcp help article text`](/reference/commands/help-article-text/) | Describe help article text. |
| [`biomcp help article citations`](/reference/commands/help-article-citations/) | Describe help article citations. |
| [`biomcp help article references`](/reference/commands/help-article-references/) | Describe help article references. |
| [`biomcp help article citation-evidence`](/reference/commands/help-article-citation-evidence/) | Describe help article citation-evidence. |
| [`biomcp help article recommendations`](/reference/commands/help-article-recommendations/) | Describe help article recommendations. |
| [`biomcp help trial`](/reference/commands/help-trial/) | Read trial groups and public documents. |
| [`biomcp help trial document`](/reference/commands/help-trial-document/) | Describe help trial document. |
| [`biomcp help variant`](/reference/commands/help-variant/) | Read source variant assertions, populations and recurrence. |
| [`biomcp help variant assertions`](/reference/commands/help-variant-assertions/) | Describe help variant assertions. |
| [`biomcp help variant population`](/reference/commands/help-variant-population/) | Describe help variant population. |
| [`biomcp help variant recurrence`](/reference/commands/help-variant-recurrence/) | Describe help variant recurrence. |
| [`biomcp help variant normalize`](/reference/commands/help-variant-normalize/) | Describe help variant normalize. |
| [`biomcp help sync`](/reference/commands/help-sync/) | Synchronize a local source bundle. |
| [`biomcp help sync hgnc`](/reference/commands/help-sync-hgnc/) | Describe help sync hgnc. |
| [`biomcp help sync clingen`](/reference/commands/help-sync-clingen/) | Describe help sync clingen. |
| [`biomcp help sync gencc`](/reference/commands/help-sync-gencc/) | Describe help sync gencc. |
| [`biomcp help sync wikipathways`](/reference/commands/help-sync-wikipathways/) | Describe help sync wikipathways. |
| [`biomcp help sync cbioportal-studies`](/reference/commands/help-sync-cbioportal-studies/) | Describe help sync cbioportal-studies. |
| [`biomcp help sync ema`](/reference/commands/help-sync-ema/) | Describe help sync ema. |
| [`biomcp help sync ddinter`](/reference/commands/help-sync-ddinter/) | Describe help sync ddinter. |
| [`biomcp help sync who-pq`](/reference/commands/help-sync-who-pq/) | Describe help sync who-pq. |
| [`biomcp help sync gtr`](/reference/commands/help-sync-gtr/) | Describe help sync gtr. |
| [`biomcp help sync who-ivd`](/reference/commands/help-sync-who-ivd/) | Describe help sync who-ivd. |
| [`biomcp help sync fda-cdx`](/reference/commands/help-sync-fda-cdx/) | Describe help sync fda-cdx. |
| [`biomcp help skill`](/reference/commands/help-skill/) | Describe help skill. |
| [`biomcp help skill list`](/reference/commands/help-skill-list/) | List supported discovery topics. |
| [`biomcp help skill render`](/reference/commands/help-skill-render/) | Describe help skill render. |
| [`biomcp help skill status`](/reference/commands/help-skill-status/) | Describe help skill status. |
| [`biomcp help skill install`](/reference/commands/help-skill-install/) | Describe help skill install. |
| [`biomcp help source`](/reference/commands/help-source/) | Describe help source. |
| [`biomcp help source list`](/reference/commands/help-source-list/) | List supported discovery topics. |
| [`biomcp help version`](/reference/commands/help-version/) | Describe help version. |
| [`biomcp help mcp-config`](/reference/commands/help-mcp-config/) | Describe help mcp-config. |
| [`biomcp help health`](/reference/commands/help-health/) | Report source terms and bounded reachability. |
| [`biomcp help store`](/reference/commands/help-store/) | Manage local cached replies and snapshots. |
| [`biomcp help store clean`](/reference/commands/help-store-clean/) | Describe help store clean. |
| [`biomcp help key`](/reference/commands/help-key/) | Register and manage source credentials without showing values. |
| [`biomcp help key list`](/reference/commands/help-key-list/) | List supported discovery topics. |
| [`biomcp help key set`](/reference/commands/help-key-set/) | Describe help key set. |
| [`biomcp help key remove`](/reference/commands/help-key-remove/) | Describe help key remove. |
| [`biomcp help serve`](/reference/commands/help-serve/) | Serve MCP over standard input or HTTP. |
| [`biomcp help schema`](/reference/commands/help-schema/) | Describe help schema. |
| [`biomcp help biomarker`](/reference/commands/help-biomarker/) | Describe help biomarker. |
| [`biomcp help papers`](/reference/commands/help-papers/) | Describe help papers. |
| [`biomcp help organization`](/reference/commands/help-organization/) | Describe help organization. |
| [`biomcp help assay-count`](/reference/commands/help-assay-count/) | Describe help assay-count. |
| [`biomcp help gwas`](/reference/commands/help-gwas/) | Describe help gwas. |
| [`biomcp help features`](/reference/commands/help-features/) | Describe help features. |
| [`biomcp help isoforms`](/reference/commands/help-isoforms/) | Describe help isoforms. |
| [`biomcp help domains`](/reference/commands/help-domains/) | Describe help domains. |
| [`biomcp help complexes`](/reference/commands/help-complexes/) | Describe help complexes. |
| [`biomcp help complex-members`](/reference/commands/help-complex-members/) | Describe help complex-members. |
| [`biomcp help structures`](/reference/commands/help-structures/) | Describe help structures. |
| [`biomcp help sequence`](/reference/commands/help-sequence/) | Describe help sequence. |
| [`biomcp help phenotype`](/reference/commands/help-phenotype/) | Describe help phenotype. |
| [`biomcp help genes`](/reference/commands/help-genes/) | Describe help genes. |
| [`biomcp help events`](/reference/commands/help-events/) | Describe help events. |
| [`biomcp help study`](/reference/commands/help-study/) | Describe help study. |
| [`biomcp help pgx`](/reference/commands/help-pgx/) | Describe help pgx. |
| [`biomcp help diagnostic`](/reference/commands/help-diagnostic/) | Describe help diagnostic. |
| [`biomcp help adverse-event`](/reference/commands/help-adverse-event/) | Describe help adverse-event. |
| [`biomcp help disease`](/reference/commands/help-disease/) | Describe help disease. |
| [`biomcp help drug`](/reference/commands/help-drug/) | Describe help drug. |
| [`biomcp help gene`](/reference/commands/help-gene/) | Describe help gene. |
| [`biomcp help supplement`](/reference/commands/help-supplement/) | Describe help supplement. |
| [`biomcp help pdf`](/reference/commands/help-pdf/) | Describe help pdf. |
| [`biomcp help text`](/reference/commands/help-text/) | Describe help text. |
| [`biomcp help citations`](/reference/commands/help-citations/) | Describe help citations. |
| [`biomcp help references`](/reference/commands/help-references/) | Describe help references. |
| [`biomcp help citation-evidence`](/reference/commands/help-citation-evidence/) | Describe help citation-evidence. |
| [`biomcp help recommendations`](/reference/commands/help-recommendations/) | Describe help recommendations. |
| [`biomcp help document`](/reference/commands/help-document/) | Describe help document. |
| [`biomcp help assertions`](/reference/commands/help-assertions/) | Describe help assertions. |
| [`biomcp help population`](/reference/commands/help-population/) | Describe help population. |
| [`biomcp help recurrence`](/reference/commands/help-recurrence/) | Describe help recurrence. |
| [`biomcp help normalize`](/reference/commands/help-normalize/) | Describe help normalize. |
| [`biomcp help help`](/reference/commands/help-help/) | Describe commands and their arguments. |
| [`biomcp help network`](/reference/commands/help-network/) | Describe help network. |
| [`biomcp help sources`](/reference/commands/help-sources/) | Describe help sources. |
| [`biomcp help config`](/reference/commands/help-config/) | Describe help config. |
| [`biomcp help relations`](/reference/commands/help-relations/) | Follow source relationships through a named get section or a search filter. |
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

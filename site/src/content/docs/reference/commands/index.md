---
title: Commands
description: Every BioMCP 1.0.0 command, grouped by entity, with its one-line
  description from the help.
goal: Find the reference page for one BioMCP command.
reader: A BioMCP user or agent looking up a command
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 has 85 commands in its help. Each links to its page with arguments, sections and examples.

## adverse-event

| Command | Description |
| --- | --- |
| [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) | Search five report headers by default. |
| [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) | Exact native report identity and an optional independently paged section. |

## article

| Command | Description |
| --- | --- |
| [`biomcp search article`](/reference/commands/search-article/) | Checked search selectors and pool cursor. |
| [`biomcp get article`](/reference/commands/get-article/) | Get one to twenty article inputs in caller order. |
| [`biomcp article supplement`](/reference/commands/article-supplement/) | Read a checked supplement file or page its original rows. |
| [`biomcp article pdf`](/reference/commands/article-pdf/) | Download one publisher PDF to an explicit file or existing folder. |
| [`biomcp article text`](/reference/commands/article-text/) | Read one bounded body part, find matching rows, or save the complete Markdown document. |
| [`biomcp article citations`](/reference/commands/article-citations/) | Page the citing or referenced PubMed papers of one checked PMID. |
| [`biomcp article references`](/reference/commands/article-references/) | Page the citing or referenced PubMed papers of one checked PMID. |
| [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/) | Read every source context and optional explicit full-text citation paragraph. |
| [`biomcp article recommendations`](/reference/commands/article-recommendations/) | Recommendations in the provider's recent-paper pool order. |

## author

| Command | Description |
| --- | --- |
| [`biomcp author papers`](/reference/commands/author-papers/) | One exact provider record's publication window. |
| [`biomcp search author`](/reference/commands/search-author/) | Literal author name query and native window; positional text maps to query. |
| [`biomcp get author`](/reference/commands/get-author/) | Exact qualified records in caller order, with no name reconciliation. |

## cell-line

| Command | Description |
| --- | --- |
| [`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/) | A separate assay-count request, never implicit in metadata or all. |
| [`biomcp search cell-line`](/reference/commands/search-cell-line/) | One literal cell line search, with native order and no total claim. |
| [`biomcp get cell-line`](/reference/commands/get-cell-line/) | One exact identity and an optional bounded named section. |

## diagnostic

| Command | Description |
| --- | --- |
| [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) | Search one diagnostic source. |
| [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) | Get one exact source record or supported section. |

## disease

| Command | Description |
| --- | --- |
| [`biomcp search disease`](/reference/commands/search-disease/) | Literal discovery text or an explicit SEER catalog page. |
| [`biomcp get disease`](/reference/commands/get-disease/) | Exact concept or native SEER site with independently paged section. |

## drug

| Command | Description |
| --- | --- |
| [`biomcp search drug`](/reference/commands/search-drug/) | Literal single-source candidate discovery, without an implicit identity selection. |
| [`biomcp get drug`](/reference/commands/get-drug/) | Exact native ID retrieval with an explicitly selected source. |

## gene

| Command | Description |
| --- | --- |
| [`biomcp get gene`](/reference/commands/get-gene/) | Get one to twenty human gene inputs in their original order. |
| [`biomcp search gene`](/reference/commands/search-gene/) | Search genes by text, cytoband, or their intersection. |

## gwas

| Command | Description |
| --- | --- |
| [`biomcp get gwas`](/reference/commands/get-gwas/) | Source association or study and native context. |
| [`biomcp search gwas`](/reference/commands/search-gwas/) | Source associations in default p-value-ascending order. |

## pathway

| Command | Description |
| --- | --- |
| [`biomcp search pathway`](/reference/commands/search-pathway/) | Search Reactome (default) or WikiPathways human pathways. |
| [`biomcp get pathway`](/reference/commands/get-pathway/) | Get an exact human Reactome R-HSA or WikiPathways WP card. |
| [`biomcp pathway genes`](/reference/commands/pathway-genes/) | Read Reactome exact references or WikiPathways GeneProduct data nodes. |
| [`biomcp pathway events`](/reference/commands/pathway-events/) | Read Reactome event objects and unresolved native integer references in source order. |

## pgx

| Command | Description |
| --- | --- |
| [`biomcp search pgx`](/reference/commands/search-pgx/) | Checked gene-keyed CPIC pair search. |
| [`biomcp get pgx`](/reference/commands/get-pgx/) | PGx card or section. |

## phenotype

| Command | Description |
| --- | --- |
| [`biomcp search phenotype`](/reference/commands/search-phenotype/) | Search literal human HP term candidates through Monarch with bounded native pages. |
| [`biomcp get phenotype`](/reference/commands/get-phenotype/) | Read an exact HP term from HPO or Monarch, or page native source sections. |

## protein

| Command | Description |
| --- | --- |
| [`biomcp protein features`](/reference/commands/protein-features/) | Page five native UniProt features by exact type and displayed residue. |
| [`biomcp protein isoforms`](/reference/commands/protein-isoforms/) | Page five source isoforms, exact-participant complexes or PDB links. |
| [`biomcp protein domains`](/reference/commands/protein-domains/) | Page five native InterPro matches with original types and fragments. |
| [`biomcp protein complexes`](/reference/commands/protein-complexes/) | Page five source isoforms, exact-participant complexes or PDB links. |
| [`biomcp protein complex-members`](/reference/commands/protein-complex-members/) | Page five native participants of an exact Complex Portal complex containing the anchor. |
| [`biomcp protein structures`](/reference/commands/protein-structures/) | Page five source isoforms, exact-participant complexes or PDB links. |
| [`biomcp protein sequence`](/reference/commands/protein-sequence/) | Read one complete native sequence for an exact source accession. |
| [`biomcp search protein`](/reference/commands/search-protein/) | Search five reviewed human UniProt headers by literal text or exact gene. |
| [`biomcp get protein`](/reference/commands/get-protein/) | Read source protein cards; a symbol returns candidates without choosing a protein. |

## study

| Command | Description |
| --- | --- |
| [`biomcp search study`](/reference/commands/search-study/) | Literal stored metadata search; text is never parsed as a query language. |
| [`biomcp get study`](/reference/commands/get-study/) | Exact case-sensitive study card or bounded metadata section. |

## trial

| Command | Description |
| --- | --- |
| [`biomcp search trial`](/reference/commands/search-trial/) | Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback. |
| [`biomcp get trial`](/reference/commands/get-trial/) | Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations. |
| [`biomcp trial document`](/reference/commands/trial-document/) | Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. |

## variant

| Command | Description |
| --- | --- |
| [`biomcp variant assertions`](/reference/commands/variant-assertions/) | Page assertions from one explicitly selected evidence source. |
| [`biomcp get variant`](/reference/commands/get-variant/) | Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. |
| [`biomcp variant population`](/reference/commands/variant-population/) | Read a bounded ancestry-detail page from the exact source-given GRCh38 allele. |
| [`biomcp search variant`](/reference/commands/search-variant/) | Discover source candidates using one declared selector, without interpreting free text. |
| [`biomcp variant recurrence`](/reference/commands/variant-recurrence/) | Observe exact-allele samples within one caller-selected source page. |
| [`biomcp variant normalize`](/reference/commands/variant-normalize/) | Send unchanged reference-qualified notation to explicitly named services. |

## Setup, keys, data and server

| Command | Description |
| --- | --- |
| [`biomcp skill`](/reference/commands/skill/) | Read embedded guidance and worked examples. |
| [`biomcp skill list`](/reference/commands/skill-list/) | List embedded worked examples. |
| [`biomcp skill render`](/reference/commands/skill-render/) | Print the canonical agent guide. |
| [`biomcp skill status`](/reference/commands/skill-status/) | Report missing, unmanaged, locally_modified, stale or current guidance. |
| [`biomcp skill install`](/reference/commands/skill-install/) | Install into DIR/skills/biomcp; a skills or biomcp directory is accepted directly. |
| [`biomcp version`](/reference/commands/version/) | Print version; --verbose adds build, schema and path provenance. |
| [`biomcp mcp-config`](/reference/commands/mcp-config/) | Print MCP configuration with this executable’s absolute path. |
| [`biomcp store clean`](/reference/commands/store-clean/) | Remove expired replies, pools and snapshots and older incompatible cache files. |
| [`biomcp health`](/reference/commands/health/) | Report publisher terms and bounded source reachability; at most one page for each implemented source. |
| [`biomcp sync`](/reference/commands/sync/) | Synchronize a local source bundle. |
| [`biomcp schema`](/reference/commands/schema/) | Print a checked-in schema artifact. |
| [`biomcp help`](/reference/commands/help/) | Print generated command help. |
| [`biomcp key list`](/reference/commands/key-list/) | List supported credential names and their origin without values. |
| [`biomcp key set`](/reference/commands/key-set/) | Register one credential from a no-echo prompt or one standard-input line. |
| [`biomcp key remove`](/reference/commands/key-remove/) | Remove a stored credential while preserving other settings. |
| [`biomcp serve`](/reference/commands/serve/) | Serve MCP over stdio or --http [ADDR]. |
| [`biomcp sync wikipathways`](/reference/commands/sync-wikipathways/) | Replace WikiPathways search/get metadata catalogs atomically; reads never sync. |
| [`biomcp sync cbioportal-studies`](/reference/commands/sync-cbioportal-studies/) | Replace the complete validated local bundle atomically. |
| [`biomcp sync ema`](/reference/commands/sync-ema/) | Replace the complete validated local bundle atomically. |
| [`biomcp sync ddinter`](/reference/commands/sync-ddinter/) | Replace the complete validated local bundle atomically. |
| [`biomcp sync who-pq`](/reference/commands/sync-who-pq/) | Replace the complete validated local bundle atomically. |
| [`biomcp sync gtr`](/reference/commands/sync-gtr/) | Replace the complete validated local bundle atomically. |
| [`biomcp sync who-ivd`](/reference/commands/sync-who-ivd/) | Replace the complete validated local bundle atomically. |
| [`biomcp sync fda-cdx`](/reference/commands/sync-fda-cdx/) | Replace the complete validated local bundle atomically. |
| [`biomcp help key ncbi`](/reference/commands/help-key-ncbi/) | Show credential registration and environment fallback names. |
| [`biomcp help key semantic-scholar`](/reference/commands/help-key-semantic-scholar/) | Show credential registration and environment fallback names. |
| [`biomcp help key orcid`](/reference/commands/help-key-orcid/) | Show credential registration and environment fallback names. |
| [`biomcp help key oncokb`](/reference/commands/help-key-oncokb/) | Show credential registration and environment fallback names. |
| [`biomcp help key nci`](/reference/commands/help-key-nci/) | Show credential registration and environment fallback names. |
| [`biomcp help key openfda`](/reference/commands/help-key-openfda/) | Show credential registration and environment fallback names. |

## Help topics

`biomcp help` does not list these topics. A reply names one in `next` when a source fails with reason `proxy` or `tls`.

| Topic | Description |
| --- | --- |
| [`biomcp help network proxy`](/reference/commands/help-network-proxy/) | Set proxy in config.json to an HTTP or HTTPS proxy URL. |
| [`biomcp help network ca_bundle`](/reference/commands/help-network-ca_bundle/) | Set ca_bundle in config.json to a PEM certificate file. |

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.

---
title: MCP server tools
description: "The three MCP tools that BioMCP 1.0.0 serves: biomcp, search and
  fetch, with their input schemas."
goal: Look up the MCP tools, their arguments and descriptions.
reader: A developer connecting an MCP client to BioMCP
kind: reference
verified: 2026-10-27
---

`biomcp serve` runs BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources, as an MCP server. An agent that connects asks it for its tools with `tools/list`, and it answers with 3, listed below with their arguments and the description each one sends. [Connect an agent](/getting-started/mcp-clients/) shows how to add the server to a client.

## biomcp

| Argument | Type | Required |
| --- | --- | --- |
| `args` | array of string | yes |

Description sent to the client, with its lines wrapped at 80 characters:

```text
help CMD;schema ID
An assay count is not an activity or sensitivity count. No treatment response is
inferred; no bioinformatics tool runs.
Author IDs name provider records;no ranking/merge. Both sources
noncommercial;--disable-sources noncommercial. Optional keys: biomcp key set
semantic-scholar (S2_API_KEY);biomcp key set orcid (ORCID_ACCESS_TOKEN)
protein features --position --type;domains
--type;isoforms;complexes;complex-members --complex;structures;sequence
Human source facts;no variant effect/structure/interaction/clinical inference;no
bioinformatics tool runs
Pages --limit --offset --pool; sections --section-limit --section-offset
Pathway membership names source/release; no therapy, response, actionability or
enrichment
GWAS source associations; no personal risk, causal variant/gene or clinical
statistic interpretation; --p-value requires default p-value-ascending
author papers details|--full --limit/-l --offset --pool :details
search author --source semantic-scholar|semanticscholar --query/-q --limit/-l
--offset --pool
get author semantic-scholar:ID|orcid:ID (semanticscholar:ID alias)
cell-line assay-count
search cell-line query --query/-q --limit/-l --offset
get cell-line CVCL_ID|SOURCE:ID --section-limit --section-offset
:names|identity|xrefs|variants|chembl|all
get gwas id section --snapshot :loci|associations|ancestries
search gwas --extended-geneset --gene --include-child-traits --p-value --pool
--pubmed query --study --trait --trait-id --variant
search protein --gene --reviewed
get protein
:features|domains|isoforms|complexes|structures|function|sequence|all
search phenotype --limit --offset --source hpo|monarch
get phenotype --section-limit --section-offset --snapshot --source hpo|monarch;
terms are terminology, associations publisher assertions; neither diagnoses
:synonyms|xrefs|hierarchy|diseases|genes
search pathway --fresh --gene --pool query --source reactome|wikipathways
--top-level
get pathway --fresh id --pool section; R-HSA|WP :summary|genes|events|all
pathway genes details id --pool; Reactome|WikiPathways GPML
pathway events id --pool; Reactome
search study --cancer-type --pool
get study :profiles|sample-lists|files|all
search pgx --cpic-level --source cpic
get pgx --diplotype --kind --source cpic|clinpgx
:recommendations|guidelines|diplotype|annotations
search diagnostic --source gtr|fda-cdx|who-ivd --biomarker --condition-id
--device-id --drug-application --gene --gene-id --manufacturer --marker --type
get diagnostic --source gtr|who-ivd|fda-pma|fda-510k --supplement
:genes|conditions|methods|regulatory|supplements
search adverse-event --reaction --received-from --received-to --serious --source
faers|maude|vaers :symptoms|seriousness|age
get adverse-event --drug-index --source faers|maude :drugs|reactions|unii|rxcui|
product-ndc|package-ndc|spl-id|spl-set-id|application-number|devices|narrative
search disease --ontology --source text
get disease :synonyms|xrefs|hierarchy|genes|phenotypes|incidence|survival
search drug
get drug
:xrefs|targets|indications|civic|label|approvals|regulatory|interactions
get gene :protein|civic|clingen|constraint|expression|pathways|oncokb|all
search gene --band --locus-group query
search article
get article :annotations|fulltext|supplements|tldr|citations|indexing|pdf|all
article supplement
article pdf
article text
article citations
article references
article citation-evidence
article recommendations
search trial
get trial :eligibility|outcomes|arms|locations|contacts|references|documents|
all|diseases|summary
trial document
variant assertions
get variant :clinvar|erepo|population|gwas|civic|oncokb|hotspots|predict|
conservation|cosmic|all
variant population
search variant
variant recurrence
variant normalize
sync wikipathways
sync cbioportal-studies
sync ema
sync ddinter
sync who-pq
sync gtr
sync who-ivd
sync fda-cdx
```

## search

| Argument | Type | Required |
| --- | --- | --- |
| `query` | string | yes |

Annotations: `destructiveHint` false, `openWorldHint` true, `readOnlyHint` true.

Description sent to the client, with its lines wrapped at 80 characters:

```text
Search BioMCP source headers. Prefix a CLI entity and its arguments, or supply
article query text.
```

## fetch

| Argument | Type | Required |
| --- | --- | --- |
| `id` | string | yes |

Annotations: `destructiveHint` false, `openWorldHint` true, `readOnlyHint` true.

Description sent to the client, with its lines wrapped at 80 characters:

```text
Fetch a search result using its exact id. Returns the CLI record as Markdown
with source metadata.
```

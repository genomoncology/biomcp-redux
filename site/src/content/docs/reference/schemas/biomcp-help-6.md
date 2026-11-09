---
title: Reply of biomcp help (biomcp.help/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp help: the fields of
  schema biomcp.help/6."
goal: "Parse the biomcp.help/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.help/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp help`](/reference/commands/help/), [`biomcp help exit`](/reference/commands/help-exit/), [`biomcp help search`](/reference/commands/help-search/), [`biomcp help search biomarker`](/reference/commands/help-search-biomarker/), [`biomcp help search author`](/reference/commands/help-search-author/), [`biomcp help search organization`](/reference/commands/help-search-organization/), [`biomcp help search cell-line`](/reference/commands/help-search-cell-line/), [`biomcp help search gwas`](/reference/commands/help-search-gwas/), [`biomcp help search protein`](/reference/commands/help-search-protein/), [`biomcp help search phenotype`](/reference/commands/help-search-phenotype/), [`biomcp help search pathway`](/reference/commands/help-search-pathway/), [`biomcp help search study`](/reference/commands/help-search-study/), [`biomcp help search pgx`](/reference/commands/help-search-pgx/), [`biomcp help search diagnostic`](/reference/commands/help-search-diagnostic/), [`biomcp help search adverse-event`](/reference/commands/help-search-adverse-event/), [`biomcp help search disease`](/reference/commands/help-search-disease/), [`biomcp help search drug`](/reference/commands/help-search-drug/), [`biomcp help search gene`](/reference/commands/help-search-gene/), [`biomcp help search article`](/reference/commands/help-search-article/), [`biomcp help search trial`](/reference/commands/help-search-trial/), [`biomcp help search variant`](/reference/commands/help-search-variant/), [`biomcp help get`](/reference/commands/help-get/), [`biomcp help get biomarker`](/reference/commands/help-get-biomarker/), [`biomcp help get author`](/reference/commands/help-get-author/), [`biomcp help get cell-line`](/reference/commands/help-get-cell-line/), [`biomcp help get gwas`](/reference/commands/help-get-gwas/), [`biomcp help get protein`](/reference/commands/help-get-protein/), [`biomcp help get phenotype`](/reference/commands/help-get-phenotype/), [`biomcp help get pathway`](/reference/commands/help-get-pathway/), [`biomcp help get study`](/reference/commands/help-get-study/), [`biomcp help get pgx`](/reference/commands/help-get-pgx/), [`biomcp help get diagnostic`](/reference/commands/help-get-diagnostic/), [`biomcp help get adverse-event`](/reference/commands/help-get-adverse-event/), [`biomcp help get disease`](/reference/commands/help-get-disease/), [`biomcp help get drug`](/reference/commands/help-get-drug/), [`biomcp help get gene`](/reference/commands/help-get-gene/), [`biomcp help get article`](/reference/commands/help-get-article/), [`biomcp help get trial`](/reference/commands/help-get-trial/), [`biomcp help get variant`](/reference/commands/help-get-variant/), [`biomcp help get clingen`](/reference/commands/help-get-clingen/), [`biomcp help author`](/reference/commands/help-author/), [`biomcp help author papers`](/reference/commands/help-author-papers/), [`biomcp help cell-line`](/reference/commands/help-cell-line/), [`biomcp help cell-line assay-count`](/reference/commands/help-cell-line-assay-count/), [`biomcp help protein`](/reference/commands/help-protein/), [`biomcp help protein features`](/reference/commands/help-protein-features/), [`biomcp help protein isoforms`](/reference/commands/help-protein-isoforms/), [`biomcp help protein domains`](/reference/commands/help-protein-domains/), [`biomcp help protein complexes`](/reference/commands/help-protein-complexes/), [`biomcp help protein complex-members`](/reference/commands/help-protein-complex-members/), [`biomcp help protein structures`](/reference/commands/help-protein-structures/), [`biomcp help protein sequence`](/reference/commands/help-protein-sequence/), [`biomcp help pathway`](/reference/commands/help-pathway/), [`biomcp help pathway genes`](/reference/commands/help-pathway-genes/), [`biomcp help pathway events`](/reference/commands/help-pathway-events/), [`biomcp help article`](/reference/commands/help-article/), [`biomcp help article supplement`](/reference/commands/help-article-supplement/), [`biomcp help article pdf`](/reference/commands/help-article-pdf/), [`biomcp help article text`](/reference/commands/help-article-text/), [`biomcp help article citations`](/reference/commands/help-article-citations/), [`biomcp help article references`](/reference/commands/help-article-references/), [`biomcp help article citation-evidence`](/reference/commands/help-article-citation-evidence/), [`biomcp help article recommendations`](/reference/commands/help-article-recommendations/), [`biomcp help trial`](/reference/commands/help-trial/), [`biomcp help trial document`](/reference/commands/help-trial-document/), [`biomcp help variant`](/reference/commands/help-variant/), [`biomcp help variant assertions`](/reference/commands/help-variant-assertions/), [`biomcp help variant population`](/reference/commands/help-variant-population/), [`biomcp help variant recurrence`](/reference/commands/help-variant-recurrence/), [`biomcp help variant normalize`](/reference/commands/help-variant-normalize/), [`biomcp help sync`](/reference/commands/help-sync/), [`biomcp help sync hgnc`](/reference/commands/help-sync-hgnc/), [`biomcp help sync clingen`](/reference/commands/help-sync-clingen/), [`biomcp help sync gencc`](/reference/commands/help-sync-gencc/), [`biomcp help sync wikipathways`](/reference/commands/help-sync-wikipathways/), [`biomcp help sync cbioportal-studies`](/reference/commands/help-sync-cbioportal-studies/), [`biomcp help sync ema`](/reference/commands/help-sync-ema/), [`biomcp help sync ddinter`](/reference/commands/help-sync-ddinter/), [`biomcp help sync who-pq`](/reference/commands/help-sync-who-pq/), [`biomcp help sync gtr`](/reference/commands/help-sync-gtr/), [`biomcp help sync who-ivd`](/reference/commands/help-sync-who-ivd/), [`biomcp help sync fda-cdx`](/reference/commands/help-sync-fda-cdx/), [`biomcp help skill`](/reference/commands/help-skill/), [`biomcp help skill list`](/reference/commands/help-skill-list/), [`biomcp help skill render`](/reference/commands/help-skill-render/), [`biomcp help skill status`](/reference/commands/help-skill-status/), [`biomcp help skill install`](/reference/commands/help-skill-install/), [`biomcp help source`](/reference/commands/help-source/), [`biomcp help source list`](/reference/commands/help-source-list/), [`biomcp help version`](/reference/commands/help-version/), [`biomcp help mcp-config`](/reference/commands/help-mcp-config/), [`biomcp help health`](/reference/commands/help-health/), [`biomcp help store`](/reference/commands/help-store/), [`biomcp help store clean`](/reference/commands/help-store-clean/), [`biomcp help serve`](/reference/commands/help-serve/), [`biomcp help schema`](/reference/commands/help-schema/), [`biomcp help biomarker`](/reference/commands/help-biomarker/), [`biomcp help papers`](/reference/commands/help-papers/), [`biomcp help organization`](/reference/commands/help-organization/), [`biomcp help assay-count`](/reference/commands/help-assay-count/), [`biomcp help gwas`](/reference/commands/help-gwas/), [`biomcp help features`](/reference/commands/help-features/), [`biomcp help isoforms`](/reference/commands/help-isoforms/), [`biomcp help domains`](/reference/commands/help-domains/), [`biomcp help complexes`](/reference/commands/help-complexes/), [`biomcp help complex-members`](/reference/commands/help-complex-members/), [`biomcp help structures`](/reference/commands/help-structures/), [`biomcp help sequence`](/reference/commands/help-sequence/), [`biomcp help phenotype`](/reference/commands/help-phenotype/), [`biomcp help genes`](/reference/commands/help-genes/), [`biomcp help events`](/reference/commands/help-events/), [`biomcp help study`](/reference/commands/help-study/), [`biomcp help pgx`](/reference/commands/help-pgx/), [`biomcp help diagnostic`](/reference/commands/help-diagnostic/), [`biomcp help adverse-event`](/reference/commands/help-adverse-event/), [`biomcp help disease`](/reference/commands/help-disease/), [`biomcp help drug`](/reference/commands/help-drug/), [`biomcp help gene`](/reference/commands/help-gene/), [`biomcp help supplement`](/reference/commands/help-supplement/), [`biomcp help pdf`](/reference/commands/help-pdf/), [`biomcp help text`](/reference/commands/help-text/), [`biomcp help citations`](/reference/commands/help-citations/), [`biomcp help references`](/reference/commands/help-references/), [`biomcp help citation-evidence`](/reference/commands/help-citation-evidence/), [`biomcp help recommendations`](/reference/commands/help-recommendations/), [`biomcp help document`](/reference/commands/help-document/), [`biomcp help assertions`](/reference/commands/help-assertions/), [`biomcp help population`](/reference/commands/help-population/), [`biomcp help recurrence`](/reference/commands/help-recurrence/), [`biomcp help normalize`](/reference/commands/help-normalize/), [`biomcp help help`](/reference/commands/help-help/), [`biomcp help network`](/reference/commands/help-network/), [`biomcp help sources`](/reference/commands/help-sources/), [`biomcp help config`](/reference/commands/help-config/), [`biomcp help relations`](/reference/commands/help-relations/), [`biomcp help network proxy`](/reference/commands/help-network-proxy/) and [`biomcp help network ca_bundle`](/reference/commands/help-network-ca_bundle/) reply with `schema` set to `biomcp.help/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.help/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_Help](#reply_help). `query` holds [Reply_HelpQuery](#reply_helpquery). `query` repeats the request as BioMCP read it.

### Reply_Help

Generated command reference from the shipped query schemas and cases.

| Field | Type | Meaning |
| --- | --- | --- |
| `commands` | list of [Reply_HelpCommand](#reply_helpcommand) | Commands available at this help path. Optional. |

### Reply_HelpQuery

Requested help path, with the root represented by an empty object.

| Field | Type | Meaning |
| --- | --- | --- |
| `command` | list of string | Command words whose help was requested. Optional. |

### Other types in this reply

3 more types appear only in this reply: [Reply_HelpArgument](#reply_helpargument), [Reply_HelpCommand](#reply_helpcommand), [Reply_HelpWord](#reply_helpword). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_HelpArgument

One schema argument, retaining its bounds and position.

| Field | Type | Meaning |
| --- | --- | --- |
| `aliases` | list of string | Alternative accepted flags. Optional. |
| `choices` | list of string | Allowed scalar values. Optional. |
| `default` | string or null | Value used when the argument is omitted. Optional. |
| `description` | string | Field documentation. |
| `maximum` | integer or null | Upper numeric or array length bound. Optional. |
| `minimum` | integer or null | Lower numeric or array length bound. Optional. |
| `name` | string | Positional name or long flag. |
| `position` | integer or null | Zero based positional index. Optional. |
| `required` | boolean | Whether the query requires this argument. |
| `value_type` | string | Schema scalar or array type. |

### Reply_HelpCommand

One command and its generated arguments.

| Field | Type | Meaning |
| --- | --- | --- |
| `arguments` | list of [Reply_HelpArgument](#reply_helpargument) | Positional arguments and flags. Optional. |
| `command` | list of string | Command words after the executable name. Optional. |
| `description` | string | Query type documentation. |
| `examples` | list of list of string | Verbatim case run steps, without replay configuration. Optional. |
| `reply_schemas` | list of string | Canonical reply contracts; empty for commands with no schema reply. Optional. |
| `section_rule` | string or null | How to disambiguate [section words](/concepts/progressive-disclosure/) from inputs. Optional. |
| `sections` | list of [Reply_HelpWord](#reply_helpword) | Accepted section words and their documentation. Optional. |

### Reply_HelpWord

A documented closed vocabulary word.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string | Type variant documentation. |
| `word` | string | Accepted spelling. |

</details>

## Next

[`biomcp help`](/reference/commands/help/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

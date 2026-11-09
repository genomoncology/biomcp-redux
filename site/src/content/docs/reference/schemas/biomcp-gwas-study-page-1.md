---
title: Reply of biomcp search gwas (biomcp.gwas-study-page/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search gwas: the
  fields of schema biomcp.gwas-study-page/1."
goal: "Parse the biomcp.gwas-study-page/1 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas-study-page/1
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search gwas`](/reference/commands/search-gwas/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.gwas-study-page/1`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.gwas-study-page/1` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_GwasStudyPage](#reply_gwasstudypage). `query` holds [Reply_GwasSearch](/reference/schemas/types/#reply_gwassearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_GwasStudyPage

One publication's native study page, without inferred clinical meaning.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_GwasSearch](/reference/schemas/types/#reply_gwassearch) or null | Same publication and study result mode. Optional. |
| `items` | list of [Reply_GwasStudyRow](#reply_gwasstudyrow) | Source study records and their native association totals. |
| `page` | [Reply_PageInfo](/reference/schemas/types/#reply_pageinfo) | Native study page scope and total. |

### Other types in this reply

One more type appear only in this reply: [Reply_GwasStudyRow](#reply_gwasstudyrow). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_GwasStudyRow

Source study metadata and the count from its association page.

| Field | Type | Meaning |
| --- | --- | --- |
| `array_manufacturers` | list of [Reply_Text](/reference/schemas/types/#reply_text) or null | Native array manufacturer labels. Optional. |
| `association_count` | integer or null | Catalog association total; absent when the count request fails. Optional. |
| `discovery_ancestries` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Native discovery ancestry summaries. Optional. |
| `genotyping_technologies` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Source technologies. Optional. |
| `gxe` | boolean or null | Native gene-environment interaction flag. Optional. |
| `gxg` | boolean or null | Native gene-gene interaction flag. Optional. |
| `id` | [Reply_GwasStudyId](/reference/schemas/types/#reply_gwasstudyid) | Exact accession. |
| `imputed` | boolean or null | Native imputation flag. Optional. |
| `initial_sample_description` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Whole discovery sample prose. Optional. |
| `license_url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native terms link without an inferred reuse grant. Optional. |
| `ontology_traits` | list of [Reply_GwasTrait](/reference/schemas/types/#reply_gwastrait) | Every native mapped ontology label and ID. Optional. |
| `platforms` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native platform description. Optional. |
| `pooled` | boolean or null | Native pooled study flag. Optional. |
| `publication` | [Reply_Pmid](/reference/schemas/types/#reply_pmid) or null | Source publication identity. Optional. |
| `replication_ancestries` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Native replication ancestry summaries. Optional. |
| `replication_sample_description` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Whole replication sample prose. Optional. |
| `reported_traits` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Native reported disease traits. Optional. |
| `snp_count` | integer or null | Source SNP count; missing differs from zero. Optional. |
| `summary_statistics_available` | boolean or null | Source availability flag; grants no download or reuse permission. Optional. |
| `summary_statistics_url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native summary statistics link; not a local download. Optional. |
| `user_requested` | boolean or null | Native user-requested study flag. Optional. |

</details>

## Next

[`biomcp search gwas`](/reference/commands/search-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

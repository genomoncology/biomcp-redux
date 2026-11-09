---
title: Reply of biomcp get gwas (biomcp.gwas-study/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas: the fields
  of schema biomcp.gwas-study/6."
goal: "Parse the biomcp.gwas-study/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas-study/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas`](/reference/commands/get-gwas/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.gwas-study/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.gwas-study/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_GwasStudy](#reply-gwasstudy). `query` holds [Reply_GwasGet](/reference/schemas/types/#reply-gwasget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_GwasStudy

Native study card; independent source descriptions are never reconciled.

| Field | Type | Meaning |
| --- | --- | --- |
| `array_manufacturers` | list of [Reply_Text](/reference/schemas/types/#reply-text) or null | Native array manufacturer labels. Optional. |
| `discovery_ancestries` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Native discovery ancestry summaries. Optional. |
| `genotyping_technologies` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Source technologies. Optional. |
| `gxe` | boolean or null | Native gene-environment interaction flag. Optional. |
| `gxg` | boolean or null | Native gene-gene interaction flag. Optional. |
| `id` | [Reply_GwasStudyId](/reference/schemas/types/#reply-gwasstudyid) | Exact accession. |
| `imputed` | boolean or null | Native imputation flag. Optional. |
| `initial_sample_description` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Whole discovery sample prose. Optional. |
| `license_url` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native terms link without an inferred reuse grant. Optional. |
| `ontology_traits` | list of [Reply_GwasTrait](/reference/schemas/types/#reply-gwastrait) | Every native mapped ontology label and ID. Optional. |
| `platforms` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native platform description. Optional. |
| `pooled` | boolean or null | Native pooled study flag. Optional. |
| `publication` | [Reply_Pmid](/reference/schemas/types/#reply-pmid) or null | Source publication identity. Optional. |
| `replication_ancestries` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Native replication ancestry summaries. Optional. |
| `replication_sample_description` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Whole replication sample prose. Optional. |
| `reported_traits` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Native reported disease traits. Optional. |
| `snp_count` | integer or null | Source SNP count; missing differs from zero. Optional. |
| `summary_statistics_available` | boolean or null | Source availability flag; grants no download or reuse permission. Optional. |
| `summary_statistics_url` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native summary statistics link; not a local download. Optional. |
| `user_requested` | boolean or null | Native user-requested study flag. Optional. |

## Next

[`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

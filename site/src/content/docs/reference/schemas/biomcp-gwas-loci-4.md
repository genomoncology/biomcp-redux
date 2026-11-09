---
title: Reply of biomcp get gwas (biomcp.gwas-loci/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas: the fields
  of schema biomcp.gwas-loci/4."
goal: "Parse the biomcp.gwas-loci/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas-loci/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas`](/reference/commands/get-gwas/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.gwas-loci/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.gwas-loci/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_GwasLoci](#reply-gwasloci). `query` holds [Reply_GwasGet](/reference/schemas/types/#reply-gwasget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_GwasLoci

One bounded locus section for an association.

| Field | Type | Meaning |
| --- | --- | --- |
| `association` | [Reply_GwasAssociationId](/reference/schemas/types/#reply-gwasassociationid) | Exact owning association. |
| `continuation` | [Reply_GwasGet](/reference/schemas/types/#reply-gwasget) | Same-resource relation continuation. Optional. |
| `items` | list of [Reply_GwasLocus](#reply-gwaslocus) | Native locus rows. Optional. |

### Other types in this reply

2 more types appear only in this reply: [Reply_GwasLocus](#reply-gwaslocus), [Reply_GwasReportedGene](#reply-gwasreportedgene). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_GwasLocus

One native locus with independent reported genes and risk-allele text.

| Field | Type | Meaning |
| --- | --- | --- |
| `author_reported_genes` | list of [Reply_GwasReportedGene](#reply-gwasreportedgene) | Reported source genes; never inferred from names. Optional. |
| `description` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Complete source locus description. Optional. |
| `source_url` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native locus self link. Optional. |
| `strongest_risk_alleles` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Source strongest-risk-allele strings. Optional. |

### Reply_GwasReportedGene

Source reported gene identity, without a causal-gene interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `ensembl_ids` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Exact source Ensembl identifiers. Optional. |
| `entrez_ids` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Exact source Entrez identifiers. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native source gene name. Optional. |

</details>

## Next

[`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

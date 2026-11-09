---
title: Reply of biomcp search gene (biomcp.gene-page/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search gene: the
  fields of schema biomcp.gene-page/8."
goal: "Parse the biomcp.gene-page/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gene-page/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search gene`](/reference/commands/search-gene/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.gene-page/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.gene-page/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_GenePage](#reply-genepage). `query` holds [Reply_GeneSearch](#reply-genesearch). `query` repeats the request as BioMCP read it.

### Reply_GenePage

One page of gene headers.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_GeneHeader](/reference/schemas/types/#reply-geneheader) | Headers after ranking and paging. Optional. |

### Reply_GeneSearch

Search genes by text, cytoband, or their intersection.

| Field | Type | Meaning |
| --- | --- | --- |
| `band` | [Reply_Cytoband](#reply-cytoband) | A cytoband prefix. Optional. |
| `limit` | integer (uint32) | Number of results per page. Optional. |
| `locus_group` | [Reply_LocusGroup](/reference/schemas/types/#reply-locusgroup) | Optional HGNC locus group intersected with the selector. Optional. |
| `offset` | integer (uint32) | Number of results to skip. Optional. |
| `query` | [Reply_Text](/reference/schemas/types/#reply-text) | Words matched against symbols, previous symbols, aliases and names. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_Cytoband](#reply-cytoband). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_Cytoband

A single nuclear cytogenetic band, without a range or location descriptor.

Type: string.

</details>

## Next

[`biomcp search gene`](/reference/commands/search-gene/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

---
title: Reply of biomcp search pathway (biomcp.pathway-page/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search pathway: the
  fields of schema biomcp.pathway-page/8."
goal: "Parse the biomcp.pathway-page/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway-page/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search pathway`](/reference/commands/search-pathway/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.pathway-page/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pathway-page/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PathwayPage](#reply_pathwaypage). `query` holds [Reply_PathwaySearch](#reply_pathwaysearch). `query` repeats the request as BioMCP read it.

### Reply_PathwayPage

Source-ordered header page; class admission never changes the native total.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_PathwayHeader](/reference/schemas/types/#reply_pathwayheader) | Whole admitted pathway headers in native order. Optional. |
| `page` | [Reply_PathwayPageInfo](/reference/schemas/types/#reply_pathwaypageinfo) | Native position and total provenance. |

### Reply_PathwaySearch

Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `fresh` | boolean | Request a new retained observation. Optional. |
| `gene` | [Reply_GeneInput](/reference/schemas/types/#reply_geneinput) | Human gene input for exact resolution and UniProt mapping; Reactome only. Optional. |
| `limit` | integer (uint32) | Five rows by default, one to fifty. Reactome rows equals this limit without refill. Optional. |
| `offset` | integer (uint32) | Native Reactome Start row or retained WikiPathways catalog offset. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) | Retained snapshot identity for deterministic continuation. Optional. |
| `query` | [Reply_Text](/reference/schemas/types/#reply_text) | Literal source query; exclusive with gene and top-level discovery. Optional. |
| `source` | list of [Reply_PathwaySource](#reply_pathwaysource) | Sources in caller order; repeated sources take turns without title joins. Optional. |
| `top_level` | boolean | Native human Reactome top-level discovery, exclusive with text and gene. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_PathwaySource](#reply_pathwaysource). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_PathwaySource

One explicit pathway source, without fallback or cross-source title joins.

Values: `reactome`, `wikipathways`.

</details>

## Next

[`biomcp search pathway`](/reference/commands/search-pathway/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

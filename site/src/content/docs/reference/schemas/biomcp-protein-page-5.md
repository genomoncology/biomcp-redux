---
title: Reply of biomcp search protein (biomcp.protein-page/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search protein: the
  fields of schema biomcp.protein-page/5."
goal: "Parse the biomcp.protein-page/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-page/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search protein`](/reference/commands/search-protein/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.protein-page/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-page/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinPage](#reply-proteinpage). `query` holds [Reply_ProteinSearch](#reply-proteinsearch). `query` repeats the request as BioMCP read it.

### Reply_ProteinPage

Source-ordered human reviewed protein headers.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinHeader](/reference/schemas/types/#reply-proteinheader) | Whole native rows in source order; totals apply only to this collection. Optional. |

### Reply_ProteinSearch

Search five reviewed human UniProt headers by literal text or exact gene. No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [Reply_GeneSymbol](/reference/schemas/types/#reply-genesymbol) | Exact native gene field, without identifier inference. Optional. |
| `limit` | integer (uint32) | Five rows by default; no more than 25. Optional. |
| `offset` | integer (uint32) | Source/retained result offset. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply-text) | Exact retained collection identity. Optional. |
| `query` | [Reply_Text](/reference/schemas/types/#reply-text) | Literal text, exclusive with gene. Optional. |
| `reviewed` | boolean | Reviewed human scope stays enabled; unreviewed search is deferred. Optional. |

## Next

[`biomcp search protein`](/reference/commands/search-protein/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

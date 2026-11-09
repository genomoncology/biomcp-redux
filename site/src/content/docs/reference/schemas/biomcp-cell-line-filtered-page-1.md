---
title: Reply of biomcp search cell-line (biomcp.cell-line-filtered-page/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search cell-line:
  the fields of schema biomcp.cell-line-filtered-page/1."
goal: "Parse the biomcp.cell-line-filtered-page/1 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell-line-filtered-page/1
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search cell-line`](/reference/commands/search-cell-line/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.cell-line-filtered-page/1`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.cell-line-filtered-page/1` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_CellLineFilteredPage](#reply-celllinefilteredpage). `query` holds [Reply_CellLineSearchCommand](#reply-celllinesearchcommand). `query` repeats the request as BioMCP read it.

### Reply_CellLineFilteredPage

Native filtered cell-line headers; the literal-search reply contract remains unchanged.

Type: [Reply_CellLinePage](/reference/schemas/types/#reply-celllinepage).

### Reply_CellLineSearchCommand

One native cell line search with optional text, species, disease, variation and derived-line filters.

| Field | Type | Meaning |
| --- | --- | --- |
| `disease` | [Reply_Text](/reference/schemas/types/#reply-text) | Cellosaurus disease: NCIt name or ID (for example Melanoma or C3224). Optional. |
| `exclude_derived` | boolean | Leave out lines derived from a parent line; needs another search term. Optional. |
| `gene` | [Reply_Text](/reference/schemas/types/#reply-text) | Cellosaurus variation gene, as the source writes it (for example KRAS). Optional. |
| `limit` | integer (uint32) | Native page size. Optional. |
| `mutation` | [Reply_Text](/reference/schemas/types/#reply-text) | Cellosaurus variation text, as the source writes it (for example p.Gly12Asp). Optional. |
| `offset` | integer (uint32) | Native start offset. Optional. |
| `protein` | [Reply_ProteinQuery](/reference/schemas/types/#reply-proteinquery) | Protein change, sent in the source's spelling (G12D is sent as p.Gly12Asp). Optional. |
| `query` | [Reply_Text](/reference/schemas/types/#reply-text) | Literal source query from the positional argument. Optional. |
| `species` | [Reply_Text](/reference/schemas/types/#reply-text) | Cellosaurus species: NCBI Taxonomy name or ID (for example Homo sapiens or 9606). Optional. |

## Next

[`biomcp search cell-line`](/reference/commands/search-cell-line/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

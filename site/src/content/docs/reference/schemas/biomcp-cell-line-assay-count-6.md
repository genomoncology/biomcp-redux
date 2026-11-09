---
title: Reply of biomcp cell-line assay-count (biomcp.cell-line-assay-count/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp cell-line
  assay-count: the fields of schema biomcp.cell-line-assay-count/6."
goal: "Parse the biomcp.cell-line-assay-count/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell-line-assay-count/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/) and [`biomcp cell-line`](/reference/commands/cell-line/) reply with `schema` set to `biomcp.cell-line-assay-count/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.cell-line-assay-count/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_CellLineAssayCount](#reply-celllineassaycount). `query` holds [Reply_CellLineAssayCountQuery](#reply-celllineassaycountquery). `query` repeats the request as BioMCP read it.

### Reply_CellLineAssayCount

Native assay total for one source-local cell identity; no activity or sensitivity count.

| Field | Type | Meaning |
| --- | --- | --- |
| `assay_count` | integer (uint64) | Native assay page total_count, including a reported zero. |
| `fetched` | integer or null | Original retrieval seconds; never a source release date. Optional. |
| `id` | [Reply_ChemblCellId](/reference/schemas/types/#reply-chemblcellid) | Exact requested ChEMBL cell ID. |
| `release` | [Reply_CellLineRelease](/reference/schemas/types/#reply-celllinerelease) or null | Independent ChEMBL release, distinct from retrieval time. Optional. |

### Reply_CellLineAssayCountQuery

A separate assay-count request, never implicit in metadata or all.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_ChemblCellId](/reference/schemas/types/#reply-chemblcellid) | Exact ChEMBL cell identity returned by metadata. |

## Next

[`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

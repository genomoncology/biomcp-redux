---
title: Reply of biomcp search cell-line (biomcp.cell-line-page/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search cell-line:
  the fields of schema biomcp.cell-line-page/7."
goal: "Parse the biomcp.cell-line-page/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell-line-page/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search cell-line`](/reference/commands/search-cell-line/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.cell-line-page/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.cell-line-page/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_CellLinePage](/reference/schemas/types/#reply-celllinepage), a shared type. `query` holds [Reply_CellLineSearch](#reply-celllinesearch). `query` repeats the request as BioMCP read it.

### Reply_CellLineSearch

One literal cell line search, with native order and no total claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Native page size. Optional. |
| `offset` | integer (uint32) | Native start offset. Optional. |
| `query` | [Reply_Text](/reference/schemas/types/#reply-text) | Literal source query from the positional argument. |

## Next

[`biomcp search cell-line`](/reference/commands/search-cell-line/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

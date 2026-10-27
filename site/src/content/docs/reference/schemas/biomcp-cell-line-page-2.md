---
title: biomcp.cell_line_page/2
description: "The BioMCP 1.0.0 reply schema biomcp.cell_line_page/2: its
  envelope fields, its result type CellLinePage and the commands that return
  it."
goal: Look up the fields of a biomcp.cell_line_page/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.cell_line_page/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.cell_line_page/2`.

These commands return it: [`biomcp search cell-line`](/reference/commands/search-cell-line/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | CellLineSearch | yes | Parsed query. |
| `result` | CellLinePage or null | no | Answer when available. |
| `schema` | "biomcp.cell_line_page/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: CellLinePage

Source-ordered cell line search headers; totals remain unknown.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fetched` | integer or null | no | Original retrieval seconds; not a source release date. |
| `items` | array of CellLineHeader | yes | Native result order, without reranking or selected identity. |
| `release` | CellLineRelease or null | no | Independently fetched publisher release, never a result total. |

## Query: CellLineSearch

One literal cell line search, with native order and no total claim.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Native page size. |
| `offset` | integer (uint32) | no | Native start offset. |
| `query` | Text | yes | Literal source query; positional spelling and --query use the same field. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.cell_line_page/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

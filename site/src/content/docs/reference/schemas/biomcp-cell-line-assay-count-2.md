---
title: biomcp.cell_line_assay_count/2
description: "The BioMCP 1.0.0 reply schema biomcp.cell_line_assay_count/2: its
  envelope fields, its result type CellLineAssayCount and the commands that
  return it."
goal: Look up the fields of a biomcp.cell_line_assay_count/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_assay_count/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.cell_line_assay_count/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.cell_line_assay_count/2`.

These commands return it: [`biomcp cell-line assay-count`](/reference/commands/cell-line-assay-count/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | CellLineAssayCountQuery | yes | Parsed query. |
| `result` | CellLineAssayCount or null | no | Answer when available. |
| `schema` | "biomcp.cell_line_assay_count/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: CellLineAssayCount

Native assay total for one source-local cell identity; no activity or sensitivity count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assay_count` | integer (uint64) | yes | Native assay page total_count, including a reported zero. |
| `fetched` | integer or null | no | Original retrieval seconds; never a source release date. |
| `id` | ChemblCellId | yes | Exact requested ChEMBL cell ID. |
| `release` | CellLineRelease or null | no | Independent ChEMBL release, distinct from retrieval time. |

## Query: CellLineAssayCountQuery

A separate assay-count request, never implicit in metadata or all.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | ChemblCellId | yes | Exact ChEMBL cell identity returned by metadata. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.cell_line_assay_count/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

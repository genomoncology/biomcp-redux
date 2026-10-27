---
title: biomcp.cell_line_identity/1
description: "The BioMCP 1.0.0 reply schema biomcp.cell_line_identity/1: its
  envelope fields, its result type CellLineIdentityPage and the commands that
  return it."
goal: Look up the fields of a biomcp.cell_line_identity/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_identity/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.cell_line_identity/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.cell_line_identity/1`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | CellLineGet | yes | Parsed query. |
| `result` | CellLineIdentityPage or null | no | Answer when available. |
| `schema` | "biomcp.cell_line_identity/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: CellLineIdentityPage

Species, diseases and warnings page independently; each total belongs to its own collection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | CellosaurusAccession | yes | Selected exact primary accession. |
| `diseases` | Capped | yes | Whole disease rows and their independent source total. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `offset` | integer (uint32) | yes | Requested offset applied independently to each collection. |
| `release` | CellLineRelease or null | no | Independently fetched publisher release. |
| `species` | Capped | yes | Whole species rows and their source total. |
| `warnings` | Capped2 | yes | Whole scoped warning rows and their independent source total. |

## Query: CellLineGet

One exact identity and an optional bounded named section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | CellLineInput | yes | One exact primary accession or qualified source ID. |
| `section` | CellLineSection | no | Omit for the default card; all returns bounded first pages without an assay count. |
| `section_limit` | integer (uint32) | no | Explicit row limit for each requested collection; native variation defaults to three. |
| `section_offset` | integer (uint32) | no | Offset within each independently requested collection. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.cell_line_identity/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

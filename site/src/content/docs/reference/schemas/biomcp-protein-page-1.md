---
title: biomcp.protein_page/1
description: "The BioMCP 1.0.0 reply schema biomcp.protein_page/1: its envelope
  fields, its result type ProteinPage and the commands that return it."
goal: Look up the fields of a biomcp.protein_page/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_page/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_page/1`.

These commands return it: [`biomcp search protein`](/reference/commands/search-protein/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ProteinSearch | yes | Parsed query. |
| `result` | ProteinPage or null | no | Answer when available. |
| `schema` | "biomcp.protein_page/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ProteinPage

Source-ordered human reviewed protein headers.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ProteinHeader | yes | Whole native rows in source order; totals apply only to this collection. |

## Query: ProteinSearch

Search five reviewed human UniProt headers by literal text or exact gene.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `gene` | GeneSymbol | no | Exact native gene field, without identifier inference. |
| `limit` | integer (uint32) | no | Five rows by default; no more than 25. |
| `offset` | integer (uint32) | no | Source/retained result offset. |
| `pool` | Text | no | Exact retained collection identity. |
| `query` | Text | no | Literal text, exclusive with gene. |
| `reviewed` | boolean | no | Reviewed human scope stays enabled; unreviewed search is deferred. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_page/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

---
title: biomcp.pgx_page/2
description: "The BioMCP 1.0.0 reply schema biomcp.pgx_page/2: its envelope
  fields, its result type PgxPage and the commands that return it."
goal: Look up the fields of a biomcp.pgx_page/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pgx_page/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pgx_page/2`.

These commands return it: [`biomcp search pgx`](/reference/commands/search-pgx/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PgxSearch | yes | Parsed query. |
| `result` | PgxPage or null | no | Answer when available. |
| `schema` | "biomcp.pgx_page/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PgxPage

Independent source-ordered page of complete native records.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `header` | PgxHeader | yes | Source and selected gene. |
| `items` | array of PgxPair | yes | Complete native rows without duplicate summaries. |
| `page` | PgxPageInfo | yes | Meaning and bounds of this collection. |

## Query: PgxSearch

Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cpic_level` | Text or null | no | Literal native level, including compound levels such as B/C. |
| `gene` | GeneSymbol | yes | Explicit source gene symbol. |
| `limit` | integer (uint32) | no | Rows per native page. |
| `offset` | integer (uint32) | no | Native source offset. |
| `source` | "cpic" | yes | Required CPIC source; ClinPGx is not supported for search. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pgx_page/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

---
title: biomcp.phenotype-search/1
description: "The BioMCP 1.0.0 reply schema biomcp.phenotype-search/1: its
  envelope fields, its result type PhenotypePage and the commands that return
  it."
goal: Look up the fields of a biomcp.phenotype-search/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-search/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.phenotype-search/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.phenotype-search/1`.

These commands return it: [`biomcp search phenotype`](/reference/commands/search-phenotype/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PhenotypeSearch | yes | Parsed query. |
| `result` | PhenotypePage or null | no | Answer when available. |
| `schema` | "biomcp.phenotype-search/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PhenotypePage

Native HP search header page, without local ranking.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of ComponentLabel | yes | Applicable publisher component terms. |
| `fetched` | integer or null | no | Original source retrieval seconds, distinct from ontology release. |
| `items` | array of PhenotypeHeader | yes | Native ordered rows; empty remains an empty collection. |

## Query: PhenotypeSearch

Search literal human HP term candidates through Monarch with bounded native pages. A term lookup answers terminology and does not establish a diagnosis.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Maximum native headers, default five. |
| `offset` | integer (uint32) | no | Native source row offset. |
| `source` | PhenotypeSource | no | Monarch only; HPO search is unsupported. |
| `text` | string | yes | Unchanged literal term search text. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.phenotype-search/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

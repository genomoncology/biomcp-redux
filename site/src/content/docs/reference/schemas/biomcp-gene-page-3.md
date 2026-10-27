---
title: biomcp.gene_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.gene_page/3: its envelope
  fields, its result type GenePage and the commands that return it."
goal: Look up the fields of a biomcp.gene_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gene_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.gene_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.gene_page/3`.

These commands return it: [`biomcp search gene`](/reference/commands/search-gene/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | GeneSearch | yes | Parsed query. |
| `result` | GenePage or null | no | Answer when available. |
| `schema` | "biomcp.gene_page/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: GenePage

One page of gene headers.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of GeneHeader | yes | Headers after ranking and paging. |

## Query: GeneSearch

Search genes by text, cytoband, or their intersection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `band` | Cytoband | no | A cytoband prefix. |
| `limit` | integer (uint32) | no | Number of results per page. |
| `locus_group` | LocusGroup | no | Optional HGNC locus group intersected with the selector. |
| `offset` | integer (uint32) | no | Number of results to skip. |
| `query` | Text | no | Words matched against symbols, previous symbols, aliases and names. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.gene_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

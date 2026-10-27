---
title: biomcp.pathway_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.pathway_page/3: its envelope
  fields, its result type PathwayPage and the commands that return it."
goal: Look up the fields of a biomcp.pathway_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pathway_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pathway_page/3`.

These commands return it: [`biomcp search pathway`](/reference/commands/search-pathway/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PathwaySearch | yes | Parsed query. |
| `result` | PathwayPage or null | no | Answer when available. |
| `schema` | "biomcp.pathway_page/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PathwayPage

Source-ordered header page; class admission never changes the native total.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of PathwayHeader | yes | Whole admitted pathway headers in native order. |
| `page` | PathwayPageInfo | yes | Native position and total provenance. |

## Query: PathwaySearch

Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fresh` | boolean | no | Request a new retained observation. |
| `gene` | GeneInput | no | Human gene input for exact resolution and UniProt mapping; Reactome only. |
| `limit` | integer (uint32) | no | Five rows by default, one to fifty. Reactome rows equals this limit without refill. |
| `offset` | integer (uint32) | no | Native Reactome Start row or retained WikiPathways catalog offset. |
| `pool` | Text | no | Retained snapshot identity for deterministic continuation. |
| `query` | Text | no | Literal source query; exclusive with gene and top-level discovery. |
| `source` | array of PathwaySource | no | Sources in caller order; repeated sources take turns without title joins. |
| `top_level` | boolean | no | Native human Reactome top-level discovery, exclusive with text and gene. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pathway_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

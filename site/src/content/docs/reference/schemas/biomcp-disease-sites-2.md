---
title: biomcp.disease_sites/2
description: "The BioMCP 1.0.0 reply schema biomcp.disease_sites/2: its envelope
  fields, its result type DiseaseSites and the commands that return it."
goal: Look up the fields of a biomcp.disease_sites/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_sites/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.disease_sites/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.disease_sites/2`.

These commands return it: [`biomcp search disease`](/reference/commands/search-disease/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | DiseaseSearch | yes | Parsed query. |
| `result` | DiseaseSites or null | no | Answer when available. |
| `schema` | "biomcp.disease_sites/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: DiseaseSites

Publisher site catalog without an ontology-to-site mapping.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of ComponentLabel | yes | Applicable component use terms, independent of row provenance. |
| `fetched` | integer or null | no | Original response retrieval time in Unix seconds, distinct from publisher version/date. |
| `items` | array of DiseaseSite | yes | Native source rows; totals in the envelope count rows, not distinct entities. |

## Query: DiseaseSearch

Literal discovery text or an explicit SEER catalog page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Maximum source rows on one page. |
| `offset` | integer (uint32) | no | Native row offset. |
| `ontology` | DiseaseOntology | no | Exactly one ontology, default MONDO. |
| `source` | DiseaseSource | no | One explicit source, default OLS. |
| `text` | string or null | no | Literal source search text; absent for the SEER site catalog. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.disease_sites/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

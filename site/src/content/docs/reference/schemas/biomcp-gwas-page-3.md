---
title: biomcp.gwas_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.gwas_page/3: its envelope
  fields, its result type GwasPage and the commands that return it."
goal: Look up the fields of a biomcp.gwas_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.gwas_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.gwas_page/3`.

These commands return it: [`biomcp search gwas`](/reference/commands/search-gwas/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | GwasPageQuery | yes | Parsed query. |
| `result` | GwasPage or null | no | Answer when available. |
| `schema` | "biomcp.gwas_page/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: GwasPage

One native association page, also used for a study associations section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | GwasPageQuery | no | Same-selector search continuation. |
| `items` | array of GwasAssociation | yes | Shared complete association records. |
| `page` | PageInfo | yes | Native page scope and total. |
| `qualifying_total` | integer or null | no | Complete qualifying count, unknown until the threshold walk reaches its end. |

## Query: GwasPageQuery

Search and study-association pages echo their original checked request.

One of:

- object: Native association search.
- object: Exact study associations relation.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.gwas_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

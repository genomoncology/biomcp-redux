---
title: biomcp.article_edges/4
description: "The BioMCP 1.0.0 reply schema biomcp.article_edges/4: its envelope
  fields, its result type ArticleEdgePage and the commands that return it."
goal: Look up the fields of a biomcp.article_edges/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_edges/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_edges/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_edges/4`.

These commands return it: [`biomcp article citations`](/reference/commands/article-citations/), [`biomcp article references`](/reference/commands/article-references/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ArticleEdges | yes | Parsed query. |
| `result` | ArticleEdgePage or null | no | Answer when available. |
| `schema` | "biomcp.article_edges/4" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ArticleEdgePage

A page of citing or referenced papers in citation order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ArticleItem | yes | Papers in the fixed edge pool. |

## Query: ArticleEdges

Page the citing or referenced PubMed papers of one checked PMID.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Maximum papers on one page. |
| `offset` | integer (uint32) | no | Number of pool items to skip. |
| `pmid` | ArticleInput | yes | PMID, PMC or DOI; iCite accepts only a PMID. |
| `pool` | Text | no | Fixed snapshot from an earlier page. |
| `source` | EdgeSource | no | Graph provider; iCite remains the default. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_edges/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

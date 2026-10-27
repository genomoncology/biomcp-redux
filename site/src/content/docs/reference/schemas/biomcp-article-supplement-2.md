---
title: biomcp.article_supplement/2
description: "The BioMCP 1.0.0 reply schema biomcp.article_supplement/2: its
  envelope fields, its result type ArticleSupplementResult and the commands that
  return it."
goal: Look up the fields of a biomcp.article_supplement/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_supplement/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_supplement/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_supplement/2`.

These commands return it: [`biomcp article supplement`](/reference/commands/article-supplement/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ArticleSupplement | yes | Parsed query. |
| `result` | ArticleSupplementResult or null | no | Answer when available. |
| `schema` | "biomcp.article_supplement/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ArticleSupplementResult

One supplement reader answer.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `content` | Section | yes | Independently licensed supplement content. |
| `pmcid` | Pmcid or null | no | Resolved PMC identity. |

## Query: ArticleSupplement

Read a checked supplement file or page its original rows.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `file` | Text | yes | Exact file name from the supplements section. |
| `find` | Text | no | Literal case-insensitive needle. |
| `input` | ArticleInput | yes | PubMed or PMC identity. |
| `offset` | integer (uint32) | no | Matching-row or text-fragment offset. |
| `replace` | boolean | no | Replace an existing destination only when explicitly requested with save. |
| `rows` | SheetRows | no | Inclusive original row range. |
| `save` | Text | no | Save the whole original document to an explicit file or existing folder. |
| `sheet` | Text | no | Sheet name for row selection. |
| `variant` | Text | no | Explicit protein substitution. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_supplement/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

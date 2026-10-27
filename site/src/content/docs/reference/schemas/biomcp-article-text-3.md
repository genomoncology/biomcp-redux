---
title: biomcp.article_text/3
description: "The BioMCP 1.0.0 reply schema biomcp.article_text/3: its envelope
  fields, its result type ArticleTextResult and the commands that return it."
goal: Look up the fields of a biomcp.article_text/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_text/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_text/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_text/3`.

These commands return it: [`biomcp article text`](/reference/commands/article-text/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ArticleText | yes | Parsed query. |
| `result` | ArticleTextResult or null | no | Answer when available. |
| `schema` | "biomcp.article_text/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ArticleTextResult

Body text response; the envelope supplies advancing reader steps.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `content` | Section | yes | Explicit section outcome including not applicable and not open. |
| `pmcid` | Pmcid or null | no | Requested PMC record when known. |

## Query: ArticleText

Read one bounded body part, find matching rows, or save the complete Markdown document.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `find` | Text | no | Case-insensitive literal text to find. |
| `input` | ArticleInput | yes | PubMed or PMC identity. DOI selectors are not accepted by the reader. |
| `offset` | integer (uint32) | no | Number of matching rows already returned. |
| `part` | integer (uint32) | no | One-based outline part number. |
| `replace` | boolean | no | Replace an existing destination only when explicitly requested with save. |
| `save` | Text | no | Save the whole original document to an explicit file or existing folder. |
| `variant` | Text | no | Explicit checked protein substitution. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_text/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

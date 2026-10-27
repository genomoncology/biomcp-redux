---
title: biomcp.article_pdf/2
description: "The BioMCP 1.0.0 reply schema biomcp.article_pdf/2: its envelope
  fields, its result type ArticlePdfResult and the commands that return it."
goal: Look up the fields of a biomcp.article_pdf/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_pdf/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_pdf/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_pdf/2`.

These commands return it: [`biomcp article pdf`](/reference/commands/article-pdf/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ArticlePdf | yes | Parsed query. |
| `result` | ArticlePdfResult or null | no | Answer when available. |
| `schema` | "biomcp.article_pdf/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ArticlePdfResult

Result of an explicit publisher PDF download.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `file` | Section | yes | Saved receipt or the reason no file was saved. |

## Query: ArticlePdf

Download one publisher PDF to an explicit file or existing folder.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | ArticleInput | yes | Article identity. |
| `replace` | boolean | no | Replace an existing file only when explicitly requested. |
| `save` | Text | yes | Destination file or existing folder. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_pdf/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

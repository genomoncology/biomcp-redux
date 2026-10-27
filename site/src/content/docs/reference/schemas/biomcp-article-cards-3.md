---
title: biomcp.article_cards/3
description: "The BioMCP 1.0.0 reply schema biomcp.article_cards/3: its envelope
  fields, its result type ArticleCards and the commands that return it."
goal: Look up the fields of a biomcp.article_cards/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_cards/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_cards/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_cards/3`.

These commands return it: [`biomcp get article`](/reference/commands/get-article/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ArticleGet | yes | Parsed query. |
| `result` | ArticleCards or null | no | Answer when available. |
| `schema` | "biomcp.article_cards/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ArticleCards

Cards in caller order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ArticleCardItem | yes | Per-input answers. |

## Query: ArticleGet

Get one to twenty article inputs in caller order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of ArticleInput | yes | PubMed, PMC or DOI identifiers in input order. |
| `section` | array of ArticleSectionWord | no | Requested section words; trailing words and --section share this field. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_cards/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

---
title: biomcp.author_page/2
description: "The BioMCP 1.0.0 reply schema biomcp.author_page/2: its envelope
  fields, its result type AuthorPage and the commands that return it."
goal: Look up the fields of a biomcp.author_page/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.author_page/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.author_page/2`.

These commands return it: [`biomcp search author`](/reference/commands/search-author/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | AuthorSearch | yes | Parsed query. |
| `result` | AuthorPage or null | no | Answer when available. |
| `schema` | "biomcp.author_page/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: AuthorPage

Native author candidates in provider order; search total counts only this query.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of AuthorHeader | yes | Source candidates; no deduplication by display name. |
| `matching_records` | integer or null | no | Native matching-record total, if supplied. |

## Query: AuthorSearch

Literal author name query and native window; positional text maps to query.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Bounded source page size. |
| `offset` | integer (uint32) | no | Native offset. |
| `pool` | Text | no | Retained page identity. |
| `query` | string | yes | Literal query from the positional argument or --query. |
| `source` | AuthorSearchSource | no | Native author source. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.author_page/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

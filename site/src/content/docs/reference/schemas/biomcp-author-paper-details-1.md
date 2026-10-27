---
title: biomcp.author_paper_details/1
description: "The BioMCP 1.0.0 reply schema biomcp.author_paper_details/1: its
  envelope fields, its result type AuthorPaperDetails and the commands that
  return it."
goal: Look up the fields of a biomcp.author_paper_details/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_paper_details/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.author_paper_details/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.author_paper_details/1`.

These commands return it: [`biomcp author papers`](/reference/commands/author-papers/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | AuthorPapersQuery | yes | Parsed query. |
| `result` | AuthorPaperDetails or null | no | Answer when available. |
| `schema` | "biomcp.author_paper_details/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: AuthorPaperDetails

Detailed metadata for the same native indexed-paper window.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `author` | AuthorId | yes | Requested provider record. |
| `items` | array of AuthorPaperDetail | yes | Original source order; no native corpus total is claimed. |

## Query: AuthorPapersQuery

One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `full` | boolean | no | Legacy alias for details; normalized on construction. |
| `id` | AuthorId | yes | Exact qualified provider record. |
| `limit` | integer (uint32) | no | Bounded row count. |
| `offset` | integer (uint32) | no | Native or retained collection offset. |
| `pool` | Text | no | Retained collection identity. |
| `section` | AuthorPaperSection | no | Detailed disclosure when requested. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.author_paper_details/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

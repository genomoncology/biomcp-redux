---
title: biomcp.author_card/3
description: "The BioMCP 1.0.0 reply schema biomcp.author_card/3: its envelope
  fields, its result type AuthorCards and the commands that return it."
goal: Look up the fields of a biomcp.author_card/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_card/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.author_card/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.author_card/3`.

These commands return it: [`biomcp get author`](/reference/commands/get-author/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | AuthorGet | yes | Parsed query. |
| `result` | AuthorCards or null | no | Answer when available. |
| `schema` | "biomcp.author_card/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: AuthorCards

Exact source cards in input order; outer source outcomes retain failures.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of AuthorCardItem | yes | Exact returned source cards, with no cross-source links. |

## Query: AuthorGet

Exact qualified records in caller order, with no name reconciliation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ids` | array of AuthorId | yes | One or more exact qualified IDs. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.author_card/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

---
title: biomcp.wikipathways_sync/1
description: "The BioMCP 1.0.0 reply schema biomcp.wikipathways_sync/1: its
  envelope fields, its result type WikiPathwaysSync and the commands that return
  it."
goal: Look up the fields of a biomcp.wikipathways_sync/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.wikipathways_sync/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.wikipathways_sync/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.wikipathways_sync/1`.

These commands return it: [`biomcp sync wikipathways`](/reference/commands/sync-wikipathways/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | SyncQuery | yes | Parsed query. |
| `result` | WikiPathwaysSync or null | no | Answer when available. |
| `schema` | "biomcp.wikipathways_sync/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: WikiPathwaysSync

Two independently validated metadata catalogs; no catalog body in stdout.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `get` | WikiPathwaysCatalogReceipt | yes | Exact-get metadata catalog receipt. |
| `search` | WikiPathwaysCatalogReceipt | yes | Search metadata catalog receipt. |

## Query: SyncQuery

Explicit source synchronization selector.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `source` | Source | yes | Source whose complete identity data is synchronized. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.wikipathways_sync/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

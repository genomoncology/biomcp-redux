---
title: biomcp.sync/4
description: "The BioMCP 1.0.0 reply schema biomcp.sync/4: its envelope fields,
  its result type SyncResult and the commands that return it."
goal: Look up the fields of a biomcp.sync/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.sync/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.sync/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.sync/4`.

These commands return it: [`biomcp sync`](/reference/commands/sync/), [`biomcp sync cbioportal-studies`](/reference/commands/sync-cbioportal-studies/), [`biomcp sync ema`](/reference/commands/sync-ema/), [`biomcp sync ddinter`](/reference/commands/sync-ddinter/), [`biomcp sync who-pq`](/reference/commands/sync-who-pq/), [`biomcp sync gtr`](/reference/commands/sync-gtr/), [`biomcp sync who-ivd`](/reference/commands/sync-who-ivd/), [`biomcp sync fda-cdx`](/reference/commands/sync-fda-cdx/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | SyncQuery | yes | Parsed query. |
| `result` | SyncResult or null | no | Answer when available. |
| `schema` | "biomcp.sync/4" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: SyncResult

Published bundle metadata after synchronization.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `bytes` | integer (uint64) | yes | Imported source-file bytes before native file decoding. |
| `duplicate_identities` | integer or null | no | Distinct repeated identities across the whole source version report. |
| `etag` | string or null | no | Native publisher ETag when supplied. |
| `last_modified` | string or null | no | Native publisher Last-Modified when supplied. |
| `rows` | integer (uint64) | yes | Imported rows. |
| `snapshot` | StudySnapshot or null | no | Catalog content revision and acquisition time, separate from source headers. |

## Query: SyncQuery

Explicit source synchronization selector.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `source` | Source | yes | Source whose complete identity data is synchronized. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.sync/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

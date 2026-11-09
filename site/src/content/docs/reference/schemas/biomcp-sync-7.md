---
title: Reply of biomcp sync (biomcp.sync/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp sync: the fields of
  schema biomcp.sync/7."
goal: "Parse the biomcp.sync/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.sync/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp sync`](/reference/commands/sync/) replies with `schema` set to `biomcp.sync/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.sync/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_SyncResult](#reply_syncresult). `query` holds [Reply_SyncQuery](/reference/schemas/types/#reply_syncquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_SyncResult

Published [bundle](/local-data/) metadata after synchronization.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Imported source-file bytes before native file decoding. |
| `data_date` | string or null | Native publisher dataset date, separate from local synchronization time. Optional. |
| `downloaded_bytes` | integer (uint64) | Source body bytes received in this sync; excludes previous partials, 304 reuse and replay. |
| `duplicate_identities` | integer or null | Distinct repeated identities across the whole source version report. Optional. |
| `etag` | string or null | Native publisher ETag when supplied. Optional. |
| `last_modified` | string or null | Native publisher Last-Modified when supplied. Optional. |
| `native_rows` | integer or null | Native catalog rows examined before identity admission. Optional. |
| `rows` | integer (uint64) | Imported rows. |
| `snapshot` | [Reply_StudySnapshot](/reference/schemas/types/#reply_studysnapshot) or null | Catalog content revision and acquisition time, separate from source headers. Optional. |

## Next

[`biomcp sync`](/reference/commands/sync/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

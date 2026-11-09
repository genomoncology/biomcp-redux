---
title: Reply of biomcp sync (biomcp.wikipathways-sync/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp sync: the fields of
  schema biomcp.wikipathways-sync/4."
goal: "Parse the biomcp.wikipathways-sync/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.wikipathways-sync/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp sync`](/reference/commands/sync/) replies with `schema` set to `biomcp.wikipathways-sync/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.wikipathways-sync/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_WikiPathwaysSync](#reply-wikipathwayssync). `query` holds [Reply_SyncQuery](/reference/schemas/types/#reply-syncquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_WikiPathwaysSync

Two independently validated metadata catalogs; no catalog body in stdout.

| Field | Type | Meaning |
| --- | --- | --- |
| `downloaded_bytes` | integer (uint64) | Source body bytes received in this synchronization; excludes replay. |
| `get` | [Reply_WikiPathwaysCatalogReceipt](#reply-wikipathwayscatalogreceipt) | Exact-get metadata catalog receipt. |
| `search` | [Reply_WikiPathwaysCatalogReceipt](#reply-wikipathwayscatalogreceipt) | Search metadata catalog receipt. |

### Other types in this reply

One more type appear only in this reply: [Reply_WikiPathwaysCatalogReceipt](#reply-wikipathwayscatalogreceipt). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_WikiPathwaysCatalogReceipt

Receipt for one independently validated WikiPathways metadata catalog.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Bytes in this catalog, not the other catalog or a GPML archive. |
| `human_rows` | integer (uint64) | Source-stated human rows in this catalog. |
| `revision` | [Reply_Text](/reference/schemas/types/#reply-text) | This catalog's retained revision. |
| `sha256` | [Reply_Text](/reference/schemas/types/#reply-text) | Hash of this catalog's retained native bytes. |
| `total_rows` | integer (uint64) | All native rows in this catalog before human filtering. |

</details>

## Next

[`biomcp sync`](/reference/commands/sync/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

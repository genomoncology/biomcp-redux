---
title: Reply of biomcp sync wikipathways (biomcp.wikipathways_sync/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp sync wikipathways: a
  real example and the fields of schema biomcp.wikipathways_sync/1."
goal: "Parse the biomcp.wikipathways_sync/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.wikipathways_sync/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp sync wikipathways`](/reference/commands/sync-wikipathways/) replies with `schema` set to `biomcp.wikipathways_sync/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp sync wikipathways
```

`biomcp schema biomcp.wikipathways_sync/1` prints the full JSON Schema.

## Example reply

`biomcp sync wikipathways` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.wikipathways_sync/1",
  "status": "complete",
  "query": {"source": "wikipathways"},
  "result": {
    "search": {
      "sha256": "5638b038a7cd19f6872c57931fc98d6a8cef98fd78619ca2151facbc7c8f…",
      "bytes": 2346682,
      "revision": "sha256:5638b038a7cd19f6872c57931fc98d6a8cef98fd78619ca2151…",
      "human_rows": 1131,
      "total_rows": 2218
    },
    "get": {
      "sha256": "6c5fc71bd7f036261a47ccaa4a71f450e54e5b3fc0537e6b85a662a74e43…",
      "bytes": 1224217,
      "revision": "sha256:6c5fc71bd7f036261a47ccaa4a71f450e54e5b3fc0537e6b85a…",
      "human_rows": 1131,
      "total_rows": 2218
    }
  },
  "sources": [
    {
      "source": "wikipathways",
      "use": "open",
      "terms": "CC0 1.0 waiver",
      "terms_url": "https://www.wikipathways.org/help.html",
      "state": "ok"
    }
  ]
}
```

## Fields

`result` holds [WikiPathwaysSync](#wikipathwayssync). `query` holds [SyncQuery](/reference/schemas/types/#syncquery), a shared type. `query` repeats the request as BioMCP read it.

### WikiPathwaysSync

Two independently validated metadata catalogs; no catalog body in stdout.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `get` | [WikiPathwaysCatalogReceipt](#wikipathwayscatalogreceipt) | yes | Exact-get metadata catalog receipt. |
| `search` | [WikiPathwaysCatalogReceipt](#wikipathwayscatalogreceipt) | yes | Search metadata catalog receipt. |

### WikiPathwaysCatalogReceipt

Receipt for one independently validated WikiPathways metadata catalog.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `bytes` | integer (uint64) | yes | Bytes in this catalog, not the other catalog or a GPML archive. |
| `human_rows` | integer (uint64) | yes | Source-stated human rows in this catalog. |
| `revision` | [Text](/reference/schemas/types/#text) | yes | This catalog's retained revision. |
| `sha256` | [Text](/reference/schemas/types/#text) | yes | Hash of this catalog's retained native bytes. |
| `total_rows` | integer (uint64) | yes | All native rows in this catalog before human filtering. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp sync wikipathways`](/reference/commands/sync-wikipathways/) gives the command's arguments.

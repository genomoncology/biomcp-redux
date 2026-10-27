---
title: Reply of biomcp store clean (biomcp.store-clean/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp store clean: a real
  example and the fields of schema biomcp.store-clean/1."
goal: "Parse the biomcp.store-clean/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.store-clean/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp store clean`](/reference/commands/store-clean/) replies with `schema` set to `biomcp.store-clean/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp store clean
```

`biomcp schema biomcp.store-clean/1` prints the full JSON Schema.

## Example reply

`biomcp store clean` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.store-clean/1",
  "status": "complete",
  "query": null,
  "result": {
    "replies": 0,
    "pools": 0,
    "items": 0,
    "snapshots": 0,
    "files": 0
  }
}
```

## Fields

`result` holds [StoreClean](#storeclean). `query` repeats the request as BioMCP read it.

### StoreClean

Counts removed by explicit local store cleanup. Fresh state and [bundles](/local-data/) remain.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `files` | integer (uint64) | yes | Older incompatible cache files. |
| `items` | integer (uint64) | yes | Items belonging to removed pools. |
| `pools` | integer (uint64) | yes | Expired search pools. |
| `replies` | integer (uint64) | yes | Expired source replies. |
| `snapshots` | integer (uint64) | yes | Expired trial and citation snapshots. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp store clean`](/reference/commands/store-clean/) gives the command's arguments.

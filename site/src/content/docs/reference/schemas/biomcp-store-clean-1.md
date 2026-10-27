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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp store clean`](/reference/commands/store-clean/) replies with `schema` set to `biomcp.store-clean/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp store clean
```

`biomcp schema biomcp.store-clean/1` prints the full JSON Schema.

## Example reply

`biomcp store clean` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [StoreClean](#storeclean). `query` repeats the request as BioMCP read it.

### StoreClean

Counts removed by explicit local store cleanup. Fresh state and [bundles](/local-data/) remain.

| Field | Type | Meaning |
| --- | --- | --- |
| `files` | integer (uint64) | Older incompatible cache files. |
| `items` | integer (uint64) | Items belonging to removed pools. |
| `pools` | integer (uint64) | Expired search pools. |
| `replies` | integer (uint64) | Expired source replies. |
| `snapshots` | integer (uint64) | Expired trial and citation snapshots. |

## Next

[`biomcp store clean`](/reference/commands/store-clean/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.

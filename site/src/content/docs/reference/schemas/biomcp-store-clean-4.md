---
title: Reply of biomcp store clean (biomcp.store-clean/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp store clean: the
  fields of schema biomcp.store-clean/4."
goal: "Parse the biomcp.store-clean/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.store-clean/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp store clean`](/reference/commands/store-clean/) and [`biomcp store`](/reference/commands/store/) reply with `schema` set to `biomcp.store-clean/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.store-clean/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_StoreClean](#reply-storeclean). `query` holds [Reply_EmptyQuery](/reference/schemas/types/#reply-emptyquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_StoreClean

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

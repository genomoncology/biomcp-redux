---
title: biomcp.store-clean/1
description: "The BioMCP 1.0.0 reply schema biomcp.store-clean/1: its envelope
  fields, its result type StoreClean and the commands that return it."
goal: Look up the fields of a biomcp.store-clean/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.store-clean/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.store-clean/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.store-clean/1`.

These commands return it: [`biomcp store clean`](/reference/commands/store-clean/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | null | yes | Parsed query. |
| `result` | StoreClean or null | no | Answer when available. |
| `schema` | "biomcp.store-clean/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: StoreClean

Counts removed by explicit local store cleanup. Fresh state and bundles remain.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `files` | integer (uint64) | yes | Older incompatible cache files. |
| `items` | integer (uint64) | yes | Items belonging to removed pools. |
| `pools` | integer (uint64) | yes | Expired search pools. |
| `replies` | integer (uint64) | yes | Expired source replies. |
| `snapshots` | integer (uint64) | yes | Expired trial and citation snapshots. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.store-clean/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

---
title: biomcp.health/2
description: "The BioMCP 1.0.0 reply schema biomcp.health/2: its envelope
  fields, its result type Health and the commands that return it."
goal: Look up the fields of a biomcp.health/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.health/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.health/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.health/2`.

These commands return it: [`biomcp health`](/reference/commands/health/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | null | yes | Parsed query. |
| `result` | Health or null | no | Answer when available. |
| `schema` | "biomcp.health/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: Health

Source reachability and publisher terms, independent of local HTTP service health.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `disable_groups` | array of DisableGroup | no | Sources and carried components excluded by each supported terms group. |
| `sources` | array of SourceHealth | yes | Every implemented adapter, in declaration order. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.health/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.

---
title: biomcp.help/2
description: "The BioMCP 1.0.0 reply schema biomcp.help/2: its envelope fields,
  its result type Help and the commands that return it."
goal: Look up the fields of a biomcp.help/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.help/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.help/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.help/2`.

These commands return it: [`biomcp help`](/reference/commands/help/), [`biomcp help network proxy`](/reference/commands/help-network-proxy/), [`biomcp help network ca_bundle`](/reference/commands/help-network-ca_bundle/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | array of string | yes | Parsed query. |
| `result` | Help or null | no | Answer when available. |
| `schema` | "biomcp.help/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: Help

Generated command reference from the shipped query schemas and cases.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `commands` | array of HelpCommand | yes | Commands available at this help path. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.help/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
